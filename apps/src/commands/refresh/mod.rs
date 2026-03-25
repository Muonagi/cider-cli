mod run;

use anyhow::{Result, anyhow};
use chrono::{Duration, Utc};
use clap::{Args, Subcommand};
use plume_core::MobileProvision;

use crate::commands::session::load_account_store;

use run::run_refreshes;

/// Next refresh time: three days before provision expiry (fallback if the date string does not parse).
pub(crate) fn scheduled_refresh_from_provision(
    provision: &MobileProvision,
) -> chrono::DateTime<Utc> {
    let expiration = provision.expiration_date();
    expiration
        .to_xml_format()
        .parse::<chrono::DateTime<Utc>>()
        .unwrap_or_else(|_| Utc::now() + Duration::days(4))
        - Duration::days(3)
}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct RefreshArgs {
    #[command(subcommand)]
    pub command: RefreshCommand,
}

#[derive(Debug, Subcommand)]
pub enum RefreshCommand {
    /// List saved refresh registrations
    List,
    /// Run refresh immediately for all or matching entries
    Run(RefreshRunArgs),
    /// Remove a refresh registration
    Remove(RefreshRemoveArgs),
}

#[derive(Debug, Args)]
pub struct RefreshRunArgs {
    /// Limit to a single device UDID
    #[arg(long, value_name = "UDID")]
    pub udid: Option<String>,
    /// Limit to a single bundle identifier
    #[arg(long, value_name = "BUNDLE_ID")]
    pub bundle_id: Option<String>,
}

#[derive(Debug, Args)]
pub struct RefreshRemoveArgs {
    /// Device UDID to remove from the refresh store
    #[arg(long, value_name = "UDID")]
    pub udid: Option<String>,
    /// Bundle identifier to remove from the selected device
    #[arg(long, value_name = "BUNDLE_ID")]
    pub bundle_id: Option<String>,
}

pub async fn execute(args: RefreshArgs) -> Result<()> {
    match args.command {
        RefreshCommand::List => list_refreshes().await,
        RefreshCommand::Run(args) => run_refreshes(args).await,
        RefreshCommand::Remove(args) => remove_refresh(args).await,
    }
}

async fn list_refreshes() -> Result<()> {
    let store = load_account_store().await?;

    if store.refreshes().is_empty() {
        println!("No refresh registrations found.");
        return Ok(());
    }

    for refresh_device in store.refreshes().values() {
        println!(
            "{}\t{}\taccount={}\tmode={}",
            refresh_device.udid,
            refresh_device.name,
            refresh_device.account,
            if refresh_device.is_mac { "mac" } else { "ios" }
        );

        for app in &refresh_device.apps {
            println!(
                "  - {}\t{}\tdue={}",
                app.bundle_id.as_deref().unwrap_or("<unknown>"),
                app.path.display(),
                app.scheduled_refresh
            );
        }
    }

    Ok(())
}

async fn remove_refresh(args: RefreshRemoveArgs) -> Result<()> {
    let mut store = load_account_store().await?;

    let udid = match args.udid {
        Some(udid) => udid,
        None => {
            let mut devices: Vec<_> = store.refreshes().keys().cloned().collect();
            devices.sort();
            devices
                .into_iter()
                .next()
                .ok_or_else(|| anyhow!("No refresh registrations found."))?
        }
    };

    if let Some(bundle_id) = args.bundle_id {
        let mut refresh_device = store
            .get_refresh_device(&udid)
            .cloned()
            .ok_or_else(|| anyhow!("No refresh registration found for device {}", udid))?;

        let mut removed_paths = Vec::new();
        refresh_device.apps.retain(|app| {
            let keep = app.bundle_id.as_deref() != Some(bundle_id.as_str());
            if !keep {
                removed_paths.push(app.path.clone());
            }
            keep
        });

        for path in removed_paths {
            let _ = tokio::fs::remove_dir_all(path).await;
        }

        if refresh_device.apps.is_empty() {
            store.remove_refresh_device(&udid).await?;
        } else {
            store.add_or_update_refresh_device(refresh_device).await?;
        }
    } else {
        if let Some(refresh_device) = store.get_refresh_device(&udid).cloned() {
            for app in refresh_device.apps {
                let _ = tokio::fs::remove_dir_all(app.path).await;
            }
        }
        store.remove_refresh_device(&udid).await?;
    }

    crate::ui::success(format!("Removed refresh registration for {}", udid));
    Ok(())
}
