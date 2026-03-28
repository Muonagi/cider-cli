use std::path::PathBuf;

use anyhow::{Context, Result, anyhow};
use clap::{Args, Subcommand};
use dialoguer::{Input, Password};

use plume_core::{AnisetteConfiguration, CertificateIdentity, auth::Account};

use crate::{
    commands::session::{
        create_session, get_selected_account, get_selected_or_named_account, load_account_store,
        resolve_team_id,
    },
    get_data_path,
};

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct AccountArgs {
    #[command(subcommand)]
    pub command: AccountCommands,
}

#[derive(Debug, Subcommand)]
#[command(arg_required_else_help = true)]
pub enum AccountCommands {
    /// Login to Apple Developer account
    Login(LoginArgs),
    /// Logout from Apple Developer account
    Logout,
    /// List all saved accounts
    List,
    /// Select the active account
    Use(SwitchArgs),
    /// Select and persist the active team for an account
    Team(TeamArgs),
    /// Export the selected team's certificate as a .p12 file
    ExportP12(ExportP12Args),
    /// List certificates for a team
    Certificates(CertificatesArgs),
    /// List devices registered to the account
    Devices(DevicesArgs),
    /// Register a new device
    RegisterDevice(RegisterDeviceArgs),
    /// List all app IDs for a team
    AppIds(AppIdsArgs),
}

#[derive(Debug, Args)]
pub struct LoginArgs {
    /// Apple ID email
    #[arg(short = 'u', long = "username", value_name = "EMAIL")]
    pub username: Option<String>,
    /// Password (will prompt if not provided)
    #[arg(short = 'p', long = "password", value_name = "PASSWORD")]
    pub password: Option<String>,
}

#[derive(Debug, Args)]
pub struct TeamArgs {
    /// Account email to manage (defaults to selected account)
    #[arg(long, value_name = "EMAIL")]
    pub email: Option<String>,
    /// Team ID to select (will prompt if not provided)
    #[arg(short = 't', long = "team", value_name = "TEAM_ID")]
    pub team_id: Option<String>,
}

#[derive(Debug, Args)]
pub struct ExportP12Args {
    /// Account email to use (defaults to selected account)
    #[arg(long, value_name = "EMAIL")]
    pub email: Option<String>,
    /// Destination path for exported .p12
    #[arg(short, long, value_name = "OUTPUT")]
    pub output: Option<PathBuf>,
}

#[derive(Debug, Args)]
pub struct CertificatesArgs {
    /// Team ID to list certificates for
    #[arg(short = 't', long = "team", value_name = "TEAM_ID")]
    pub team_id: Option<String>,
}

#[derive(Debug, Args)]
pub struct DevicesArgs {
    /// Team ID to list devices for
    #[arg(short = 't', long = "team", value_name = "TEAM_ID")]
    pub team_id: Option<String>,
}

#[derive(Debug, Args)]
pub struct RegisterDeviceArgs {
    /// Team ID to register the device under
    #[arg(short = 't', long = "team", value_name = "TEAM_ID")]
    pub team_id: Option<String>,
    /// Device UDID
    #[arg(short = 'u', long = "udid", value_name = "UDID", required = true)]
    pub udid: String,
    /// Device name
    #[arg(short = 'n', long = "name", value_name = "NAME", required = true)]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct AppIdsArgs {
    /// Team ID to list app IDs for
    #[arg(short = 't', long = "team", value_name = "TEAM_ID")]
    pub team_id: Option<String>,
}

#[derive(Debug, Args)]
pub struct SwitchArgs {
    /// Email of the account to switch to
    #[arg(value_name = "EMAIL", required = true)]
    pub email: String,
}

pub async fn execute(args: AccountArgs) -> Result<()> {
    match args.command {
        AccountCommands::Login(login_args) => login(login_args).await,
        AccountCommands::Logout => logout().await,
        AccountCommands::List => list_accounts().await,
        AccountCommands::Use(switch_args) => switch_account(switch_args).await,
        AccountCommands::Team(team_args) => select_team(team_args).await,
        AccountCommands::ExportP12(export_args) => export_p12(export_args).await,
        AccountCommands::Certificates(cert_args) => certificates(cert_args).await,
        AccountCommands::Devices(device_args) => devices(device_args).await,
        AccountCommands::RegisterDevice(register_args) => register_device(register_args).await,
        AccountCommands::AppIds(app_id_args) => app_ids(app_id_args).await,
    }
}

async fn login(args: LoginArgs) -> Result<()> {
    let tfa_closure = || -> std::result::Result<String, String> {
        Input::<String>::new()
            .with_prompt("Enter 2FA code")
            .interact_text()
            .map_err(|e| e.to_string())
    };

    let anisette_config = AnisetteConfiguration::default().set_configuration_path(get_data_path());

    let username = match args.username {
        Some(user) => user,
        None => Input::<String>::new()
            .with_prompt("Apple ID email")
            .interact_text()?,
    };

    let password = match args.password {
        Some(pass) => pass,
        None => Password::new().with_prompt("Password").interact()?,
    };

    let login_closure = || -> std::result::Result<(String, String), String> {
        Ok((username.clone(), password.clone()))
    };

    let sp = crate::ui::spinner("Logging in...");
    let account = Account::login(login_closure, tfa_closure, anisette_config).await?;

    let mut settings = load_account_store().await?;
    settings
        .accounts_add_from_session(username, account)
        .await?;

    crate::ui::finish_spinner(&sp, "Logged in and account saved");

    Ok(())
}

async fn logout() -> Result<()> {
    let mut settings = load_account_store().await?;

    let email = settings
        .selected_account()
        .ok_or_else(|| anyhow!("No account currently logged in"))?
        .email()
        .clone();

    settings.accounts_remove(&email).await?;

    crate::ui::success("Logged out and removed account");

    Ok(())
}

async fn select_team(args: TeamArgs) -> Result<()> {
    let account = get_selected_or_named_account(args.email.as_deref()).await?;
    let session = create_session(&account).await?;
    let team_id = resolve_team_id(&session, &account, args.team_id.as_deref(), true)?;

    let mut settings = load_account_store().await?;
    settings
        .update_account_team(account.email(), team_id.clone())
        .await?;

    crate::ui::success(format!("Selected team {} for {}", team_id, account.email()));

    Ok(())
}

async fn export_p12(args: ExportP12Args) -> Result<()> {
    let account = get_selected_or_named_account(args.email.as_deref()).await?;
    let session = create_session(&account).await?;
    let team_id = resolve_team_id(&session, &account, None, false)?;

    let identity =
        CertificateIdentity::new_with_session(&session, get_data_path(), None, &team_id, true)
            .await?;

    let p12_data = identity
        .p12_data
        .context("Certificate export did not return .p12 data")?;

    let output = args
        .output
        .unwrap_or_else(|| get_data_path().join(format!("{team_id}_certificate.p12")));

    tokio::fs::write(&output, p12_data).await?;
    crate::ui::success(format!("Saved certificate to {}", output.display()));

    Ok(())
}

async fn certificates(args: CertificatesArgs) -> Result<()> {
    let account = get_selected_account().await?;
    let session = create_session(&account).await?;
    let team_id = resolve_team_id(&session, &account, args.team_id.as_deref(), false)?;

    let certs = session.qh_list_certs(&team_id).await?.certificates;

    if certs.is_empty() {
        crate::ui::status("No certificates found.");
        return Ok(());
    }

    crate::ui::header(format!("Certificates ({})", certs.len()));
    for (i, cert) in certs.iter().enumerate() {
        if i > 0 {
            println!();
        }
        let status_style = match cert.status.as_str() {
            "Issued" => console::style(&cert.status).green(),
            "Revoked" | "Expired" => console::style(&cert.status).red(),
            _ => console::style(&cert.status).yellow(),
        };
        let type_name = cert
            .cert_type
            .as_ref()
            .map(|t| t.name.as_str())
            .unwrap_or("Unknown");
        println!(
            "  {} {}",
            console::style(&cert.name).bold(),
            console::style(type_name).dim(),
        );
        println!(
            "    {} {} {} {} {}",
            console::style(&cert.serial_number).dim(),
            console::style("·").dim(),
            status_style,
            console::style("·").dim(),
            console::style(cert.expiration_date.to_xml_format()).dim(),
        );
        if let Some(machine) = &cert.machine_name {
            println!("    {}", console::style(machine).dim());
        }
    }

    Ok(())
}

async fn devices(args: DevicesArgs) -> Result<()> {
    let account = get_selected_account().await?;
    let session = create_session(&account).await?;
    let team_id = resolve_team_id(&session, &account, args.team_id.as_deref(), false)?;

    let devices = session.qh_list_devices(&team_id).await?.devices;

    if devices.is_empty() {
        crate::ui::status("No devices found.");
        return Ok(());
    }

    crate::ui::header(format!("Devices ({})", devices.len()));
    for (i, device) in devices.iter().enumerate() {
        if i > 0 {
            println!();
        }
        println!(
            "  {} {}",
            console::style(&device.name).bold(),
            console::style(&device.device_class).dim(),
        );
        println!(
            "    {} {} {}",
            console::style(&device.device_number).dim(),
            console::style("·").dim(),
            device.status,
        );
    }

    Ok(())
}

async fn register_device(args: RegisterDeviceArgs) -> Result<()> {
    let account = get_selected_account().await?;
    let session = create_session(&account).await?;
    let team_id = resolve_team_id(&session, &account, args.team_id.as_deref(), false)?;

    let device = session
        .qh_add_device(&team_id, &args.name, &args.udid)
        .await?
        .device;

    crate::ui::success(format!(
        "Registered device: {} ({})",
        device.name, device.device_number
    ));

    Ok(())
}

async fn app_ids(args: AppIdsArgs) -> Result<()> {
    let account = get_selected_account().await?;
    let session = create_session(&account).await?;
    let team_id = resolve_team_id(&session, &account, args.team_id.as_deref(), false)?;

    let app_ids = session.v1_list_app_ids(&team_id, None).await?.data;

    if app_ids.is_empty() {
        crate::ui::status("No app IDs found.");
        return Ok(());
    }

    crate::ui::header(format!("App IDs ({})", app_ids.len()));
    for (i, app) in app_ids.iter().enumerate() {
        if i > 0 {
            println!();
        }
        println!(
            "  {} {}",
            console::style(&app.attributes.name).bold(),
            console::style(&app.id).dim(),
        );
        println!("    {}", console::style(&app.attributes.identifier).dim());
    }

    Ok(())
}

async fn list_accounts() -> Result<()> {
    let settings = load_account_store().await?;

    let accounts = settings.accounts();

    if accounts.is_empty() {
        crate::ui::status("No accounts found. Use 'cider account login' to add an account.");
        return Ok(());
    }

    let selected_email = settings
        .selected_account()
        .map(|account| account.email().clone());

    crate::ui::header("Saved accounts");
    for (email, account) in accounts {
        let marker = if Some(email) == selected_email.as_ref() {
            console::style("●").green().to_string()
        } else {
            console::style("○").dim().to_string()
        };

        let team_display = if account.team_id().is_empty() {
            String::new()
        } else {
            format!(
                " {} {}",
                console::style("·").dim(),
                console::style(account.team_id()).dim()
            )
        };

        println!(
            "  {} {} {}{}",
            marker,
            email,
            console::style(account.first_name()).dim(),
            team_display,
        );
    }

    Ok(())
}

async fn switch_account(args: SwitchArgs) -> Result<()> {
    let mut settings = load_account_store().await?;

    if settings.get_account(&args.email).is_none() {
        return Err(anyhow!(
            "Account '{}' not found. Use 'cider account list' to see available accounts.",
            args.email
        ));
    }

    settings.account_select(&args.email).await?;

    crate::ui::success(format!("Switched to account: {}", args.email));

    Ok(())
}
