use std::collections::HashMap;

use anyhow::{Result, anyhow};
use plume_core::{CertificateIdentity, MobileProvision, developer::DeveloperSession};
use plume_store::{AccountStore, RefreshApp, RefreshDevice};
use plume_utils::{Bundle, Device, Signer, SignerMode, SignerOptions};

use crate::{
    commands::{
        device::list_connected_devices,
        session::{create_session, load_account_store, resolve_team_id},
    },
    get_data_path,
};

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
use crate::commands::device::mac_local_device;

use super::{RefreshRunArgs, scheduled_refresh_from_provision};

pub(super) async fn run_refreshes(args: RefreshRunArgs) -> Result<()> {
    let mut store = load_account_store().await?;
    let devices = connected_refresh_targets().await;

    let target_udids: Vec<String> = match args.udid {
        Some(udid) => vec![udid],
        None => store.refreshes().keys().cloned().collect(),
    };

    if target_udids.is_empty() {
        println!("No refresh registrations found.");
        return Ok(());
    }

    for udid in target_udids {
        let refresh_device = store
            .get_refresh_device(&udid)
            .cloned()
            .ok_or_else(|| anyhow!("No refresh registration found for device {}", udid))?;

        let device = devices
            .get(&udid)
            .cloned()
            .ok_or_else(|| anyhow!("Device {} is not currently available for refresh", udid))?;

        let app_indices = matching_app_indices(&refresh_device, args.bundle_id.as_deref())?;

        for app_index in app_indices {
            let app = refresh_device
                .apps
                .get(app_index)
                .cloned()
                .ok_or_else(|| anyhow!("Refresh app index {} no longer exists", app_index))?;

            refresh_one(&mut store, &refresh_device, &app, &device).await?;
        }
    }

    Ok(())
}

fn matching_app_indices(
    refresh_device: &RefreshDevice,
    bundle_id: Option<&str>,
) -> Result<Vec<usize>> {
    let matching: Vec<usize> = refresh_device
        .apps
        .iter()
        .enumerate()
        .filter_map(|(index, app)| {
            if let Some(bundle_id) = bundle_id {
                (app.bundle_id.as_deref() == Some(bundle_id)).then_some(index)
            } else {
                Some(index)
            }
        })
        .collect();

    if matching.is_empty() {
        if let Some(bundle_id) = bundle_id {
            return Err(anyhow!(
                "No refresh registration found for bundle '{}' on device {}",
                bundle_id,
                refresh_device.udid
            ));
        }
    }

    Ok(matching)
}

async fn connected_refresh_targets() -> HashMap<String, Device> {
    let mut devices = HashMap::new();

    if let Ok(connected) = list_connected_devices().await {
        for device in connected {
            devices.insert(device.udid.clone(), device);
        }
    }

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        devices.insert("mac".to_string(), mac_local_device());
    }

    devices
}

async fn refresh_one(
    store: &mut AccountStore,
    refresh_device: &RefreshDevice,
    app: &RefreshApp,
    device: &Device,
) -> Result<()> {
    crate::ui::header(format!(
        "Refreshing {} for {}",
        app.bundle_id.as_deref().unwrap_or("unknown"),
        refresh_device.name
    ));

    let account = store
        .get_account(&refresh_device.account)
        .cloned()
        .ok_or_else(|| anyhow!("Account {} not found", refresh_device.account))?;

    let session = create_session(&account).await?;
    let team_id = resolve_team_id(&session, &account, None, false)?;

    let sp = crate::ui::spinner("Checking certificate...");
    let identity =
        CertificateIdentity::new_with_session(&session, get_data_path(), None, &team_id, false)
            .await?;
    let identity_is_new = identity.new;
    crate::ui::finish_spinner(&sp, "Certificate checked");

    let is_installed = if let Some(bundle_id) = app.bundle_id.as_deref() {
        if device.is_mac {
            false
        } else {
            device.is_app_installed(bundle_id).await?
        }
    } else {
        false
    };

    let needs_reinstall = device.is_mac || identity_is_new || !is_installed;

    if needs_reinstall {
        resign_and_reinstall(app, device, &session, &team_id, identity).await?;
    } else {
        update_provisioning_profiles(app, device, &session, &team_id).await?;
    }

    update_refresh_schedule(store, refresh_device, app).await?;

    Ok(())
}

async fn resign_and_reinstall(
    app: &RefreshApp,
    device: &Device,
    session: &DeveloperSession,
    team_id: &str,
    signing_identity: CertificateIdentity,
) -> Result<()> {
    if should_register_portal_device(device) {
        session
            .qh_ensure_device(&team_id.to_string(), &device.name, &device.udid)
            .await?;
    }

    let bundle = Bundle::new(app.path.clone())?;
    let team_id = team_id.to_string();
    let mut signer = Signer::new(
        Some(signing_identity),
        SignerOptions {
            mode: SignerMode::Pem,
            ..Default::default()
        },
    );

    let sp = crate::ui::spinner("Registering bundle...");
    signer
        .register_bundle(&bundle, session, &team_id, true)
        .await?;
    crate::ui::finish_spinner(&sp, "Bundle registered");

    let sp = crate::ui::spinner("Signing bundle...");
    signer.sign_bundle(&bundle).await?;
    crate::ui::finish_spinner(&sp, "Bundle signed");

    if device.is_mac {
        let sp = crate::ui::spinner("Installing to Mac...");
        plume_utils::install_app_mac(&app.path).await?;
        crate::ui::finish_spinner(&sp, "Installed to Mac");
    } else {
        let pb = crate::ui::progress_bar(100);
        let pb_clone = pb.clone();
        device
            .install_app(&app.path, move |progress| {
                let pb = pb_clone.clone();
                async move {
                    pb.set_position(progress as u64);
                }
            })
            .await?;
        pb.finish_and_clear();
        crate::ui::success(format!("Installed to {}", device.name));
    }

    Ok(())
}

async fn update_provisioning_profiles(
    app: &RefreshApp,
    device: &Device,
    session: &DeveloperSession,
    team_id: &str,
) -> Result<()> {
    let bundle = Bundle::new(app.path.clone())?;
    let mut signer = Signer::new(
        None,
        SignerOptions {
            mode: SignerMode::Pem,
            ..Default::default()
        },
    );

    let sp = crate::ui::spinner("Updating provisioning profiles...");
    signer
        .register_bundle(&bundle, session, &team_id.to_string(), true)
        .await?;

    for provision in &signer.provisioning_files {
        device.install_profile(provision).await?;
    }
    crate::ui::finish_spinner(&sp, "Provisioning profiles updated");

    Ok(())
}

async fn update_refresh_schedule(
    store: &mut AccountStore,
    refresh_device: &RefreshDevice,
    app: &RefreshApp,
) -> Result<()> {
    let embedded_provision = app.path.join("embedded.mobileprovision");
    if !embedded_provision.exists() {
        return Err(anyhow!(
            "embedded.mobileprovision not found at {}",
            embedded_provision.display()
        ));
    }

    let provision = MobileProvision::load_with_path(&embedded_provision)?;
    let scheduled_refresh = scheduled_refresh_from_provision(&provision);

    let mut updated_device = refresh_device.clone();
    if let Some(existing_app) = updated_device
        .apps
        .iter_mut()
        .find(|item| item.path == app.path)
    {
        existing_app.scheduled_refresh = scheduled_refresh;
    }

    store.add_or_update_refresh_device(updated_device).await?;
    Ok(())
}

fn should_register_portal_device(device: &Device) -> bool {
    !device.is_mac
}
