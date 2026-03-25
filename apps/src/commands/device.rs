use std::path::PathBuf;

use anyhow::{Result, anyhow};
use clap::{Args, Subcommand};
use dialoguer::Select;
use plume_utils::{Device, Package, SignerAppReal};

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct DeviceArgs {
    #[command(subcommand)]
    pub command: DeviceCommand,
}

#[derive(Debug, Subcommand)]
pub enum DeviceCommand {
    /// List connected devices
    List,
    /// Pair and trust a connected device
    Trust(DeviceTargetArgs),
    /// List supported installed apps on a device
    Apps(DeviceTargetArgs),
    /// Install an app bundle or package onto a device
    Install(InstallArgs),
    /// Install the host pairing record into an app's documents directory
    Pairing(PairingArgs),
}

#[derive(Debug, Args, Clone)]
pub struct DeviceTargetArgs {
    /// Target a specific connected device by UDID
    #[arg(
        short = 'u',
        long = "udid",
        value_name = "UDID",
        conflicts_with = "mac"
    )]
    pub udid: Option<String>,
    /// Target the connected Apple Silicon Mac
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    #[arg(short = 'm', long = "mac", conflicts_with = "udid")]
    pub mac: bool,
}

#[derive(Debug, Args)]
pub struct InstallArgs {
    #[command(flatten)]
    pub target: DeviceTargetArgs,
    /// Path to an .ipa or .app bundle
    #[arg(value_name = "PATH")]
    pub path: PathBuf,
}

#[derive(Debug, Args)]
pub struct PairingArgs {
    #[command(flatten)]
    pub target: DeviceTargetArgs,
    /// Bundle identifier of the installed app to receive the pairing file
    #[arg(long = "app", value_name = "IDENTIFIER")]
    pub app_identifier: Option<String>,
    /// Override the pairing file path inside the app container
    #[arg(long = "path", value_name = "PATH")]
    pub pairing_path: Option<PathBuf>,
}

pub async fn execute(args: DeviceArgs) -> Result<()> {
    match args.command {
        DeviceCommand::List => list_devices().await,
        DeviceCommand::Trust(target) => trust_device(target).await,
        DeviceCommand::Apps(target) => list_apps(target).await,
        DeviceCommand::Install(args) => install_app(args).await,
        DeviceCommand::Pairing(args) => install_pairing(args).await,
    }
}

/// Synthetic device entry for “install to this Mac” flows on Apple Silicon.
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
pub(crate) fn mac_local_device() -> Device {
    Device {
        name: "My Mac".to_string(),
        udid: "mac".to_string(),
        device_id: 0,
        usbmuxd_device: None,
        is_mac: true,
    }
}

pub async fn list_connected_devices() -> Result<Vec<Device>> {
    let mut muxer = idevice::usbmuxd::UsbmuxdConnection::default().await?;
    let raw_devices = muxer.get_devices().await?;
    let device_futures: Vec<_> = raw_devices.into_iter().map(Device::new).collect();
    Ok(futures::future::join_all(device_futures).await)
}

async fn list_devices() -> Result<()> {
    let devices = list_connected_devices().await?;

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        println!("mac\tMy Mac\tlocal");
    }

    if devices.is_empty() {
        println!("No connected iOS devices found.");
        return Ok(());
    }

    for device in devices {
        println!("{}\t{}\t{}", device.udid, device.name, device);
    }

    Ok(())
}

pub async fn select_device(device_udid: Option<String>) -> Result<Device> {
    let devices = list_connected_devices().await?;

    if let Some(udid) = device_udid {
        return devices
            .into_iter()
            .find(|device| device.udid == udid || device.device_id.to_string() == udid)
            .ok_or_else(|| anyhow!("No connected device matched '{}'", udid));
    }

    if devices.is_empty() {
        return Err(anyhow!(
            "No devices connected. Please connect a device and try again."
        ));
    }

    let device_names: Vec<String> = devices
        .iter()
        .map(|device| format!("{} ({})", device, device.udid))
        .collect();

    let selection = Select::new()
        .with_prompt("Select a device")
        .items(&device_names)
        .default(0)
        .interact()?;

    Ok(devices[selection].clone())
}

pub async fn select_target(args: &DeviceTargetArgs) -> Result<Device> {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        if args.mac {
            return Ok(mac_local_device());
        }
    }

    select_device(args.udid.clone()).await
}

async fn trust_device(target: DeviceTargetArgs) -> Result<()> {
    let device = select_target(&target).await?;

    if device.is_mac {
        return Err(anyhow!(
            "Trust is only supported for connected iOS devices."
        ));
    }

    let sp = crate::ui::spinner(format!("Pairing {}...", device.name));
    device.pair().await?;
    crate::ui::finish_spinner(&sp, format!("Paired device {}", device.name));
    Ok(())
}

async fn list_apps(target: DeviceTargetArgs) -> Result<()> {
    let device = select_target(&target).await?;

    if device.is_mac {
        return Err(anyhow!(
            "App listing is only supported for connected iOS devices."
        ));
    }

    let apps = device.installed_apps().await?;

    if apps.is_empty() {
        println!("No supported installed apps found.");
        return Ok(());
    }

    for app in apps {
        println!(
            "{}\t{}",
            app.bundle_id.unwrap_or_else(|| "<unknown>".to_string()),
            app.app
        );
    }

    Ok(())
}

async fn install_app(args: InstallArgs) -> Result<()> {
    let device = select_target(&args.target).await?;
    let mut app_path = args.path;

    if !app_path.is_dir() {
        app_path = Package::new(app_path)?
            .get_package_bundle()?
            .bundle_dir()
            .clone();
    }

    if device.is_mac {
        let sp = crate::ui::spinner("Installing to Mac...");
        plume_utils::install_app_mac(&app_path).await?;
        crate::ui::finish_spinner(&sp, "Installed to Mac");
        return Ok(());
    }

    let pb = crate::ui::progress_bar(100);
    let pb_clone = pb.clone();
    device
        .install_app(&app_path, move |progress| {
            let pb = pb_clone.clone();
            async move {
                pb.set_position(progress as u64);
            }
        })
        .await?;
    pb.finish_and_clear();
    crate::ui::success(format!("Installed to {}", device.name));

    Ok(())
}

async fn install_pairing(args: PairingArgs) -> Result<()> {
    let device = select_target(&args.target).await?;

    if device.is_mac {
        return Err(anyhow!(
            "Pairing-file installation is only supported for iOS devices."
        ));
    }

    let app = if let Some(identifier) = args.app_identifier {
        SignerAppReal::from_bundle_identifier(Some(identifier.as_str()))
    } else {
        select_pairing_app(&device).await?
    };

    let bundle_id = app
        .bundle_id
        .clone()
        .ok_or_else(|| anyhow!("Selected app is missing a bundle identifier"))?;

    let pairing_path = match args.pairing_path {
        Some(path) => path,
        None => PathBuf::from(
            app.app
                .pairing_file_path()
                .ok_or_else(|| anyhow!("No default pairing path is known for {}", app.app))?,
        ),
    };

    let sp = crate::ui::spinner("Installing pairing record...");
    device
        .install_pairing_record(&bundle_id, &pairing_path.to_string_lossy())
        .await?;

    crate::ui::finish_spinner(&sp, format!("Installed pairing record for {}", bundle_id));

    Ok(())
}

async fn select_pairing_app(device: &Device) -> Result<SignerAppReal> {
    let apps = device.installed_apps().await?;

    if apps.is_empty() {
        return Err(anyhow!("No supported apps were found on the device."));
    }

    let labels: Vec<String> = apps
        .iter()
        .map(|app| {
            format!(
                "{} ({})",
                app.app,
                app.bundle_id.as_deref().unwrap_or("unknown")
            )
        })
        .collect();

    let selection = Select::new()
        .with_prompt("Select an app to receive the pairing file")
        .items(&labels)
        .default(0)
        .interact()?;

    Ok(apps[selection].clone())
}
