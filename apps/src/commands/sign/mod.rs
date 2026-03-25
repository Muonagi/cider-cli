mod install;
mod signer;

use std::path::PathBuf;

use anyhow::{Result, anyhow};
use clap::Args;

use plume_core::MobileProvision;
use plume_utils::{SignerInstallMode, SignerMode};

use crate::commands::{
    device::{DeviceTargetArgs, select_target},
};

use install::{install_signed_bundle, load_store, maybe_install_pairing, save_refresh_registration};
use signer::{
    build_signer, build_signer_options, load_input, resolve_post_sign_action, resolve_sign_mode,
};

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct SignArgs {
    /// Path to the app bundle or package to sign (.app or .ipa)
    #[arg(value_name = "PACKAGE")]
    pub package: PathBuf,
    /// PEM files for certificate and private key
    #[arg(long = "pem", value_name = "PEM", num_args = 1..)]
    pub pem_files: Option<Vec<PathBuf>>,
    /// Use the selected Apple ID account
    #[arg(long = "apple-id")]
    pub apple_id: bool,
    /// Perform ad-hoc signing (no certificate required)
    #[arg(long)]
    pub adhoc: bool,
    /// Skip bundle modification and signing, only extract/install/export
    #[arg(long = "no-modify")]
    pub no_modify: bool,
    /// Provisioning profile file to embed
    #[arg(long = "provision", value_name = "PROVISION")]
    pub provisioning_file: Option<PathBuf>,
    /// Custom bundle identifier to set
    #[arg(long = "custom-identifier", value_name = "BUNDLE_ID")]
    pub bundle_identifier: Option<String>,
    /// Custom bundle name to set
    #[arg(long = "custom-name", value_name = "NAME")]
    pub name: Option<String>,
    /// Custom bundle version to set
    #[arg(long = "custom-version", value_name = "VERSION")]
    pub version: Option<String>,
    /// Replace app icons with the provided image
    #[arg(long = "custom-icon", value_name = "IMAGE")]
    pub custom_icon: Option<PathBuf>,
    /// Custom entitlements plist to embed when single-profile mode is enabled
    #[arg(long = "custom-entitlements", value_name = "PLIST")]
    pub custom_entitlements: Option<PathBuf>,
    /// Support older iOS versions by forcing MinimumOSVersion=7.0
    #[arg(long = "support-minimum-os-version")]
    pub support_minimum_os_version: bool,
    /// Force file sharing entitlements
    #[arg(long = "file-sharing")]
    pub support_file_sharing: bool,
    /// Force iPad fullscreen mode
    #[arg(long = "ipad-fullscreen")]
    pub support_ipad_fullscreen: bool,
    /// Force Game Mode support
    #[arg(long = "game-mode")]
    pub support_game_mode: bool,
    /// Force ProMotion support
    #[arg(long = "pro-motion")]
    pub support_pro_motion: bool,
    /// Force Liquid Glass compatibility tweaks
    #[arg(long = "liquid-glass")]
    pub support_liquid_glass: bool,
    /// Replace Substrate with ElleKit
    #[arg(long = "ellekit")]
    pub support_ellekit: bool,
    /// Only register the main bundle when requesting provisioning
    #[arg(long = "single-profile")]
    pub single_profile: bool,
    /// Remember the signed app for future refreshes
    #[arg(long = "refresh")]
    pub refresh: bool,
    /// Additional tweak files to inject (.deb, .dylib)
    #[arg(long = "tweak", value_name = "PATH", num_args = 1..)]
    pub tweaks: Option<Vec<PathBuf>>,
    /// Additional bundle-like payloads to inject (.framework, .bundle, .appex)
    #[arg(long = "bundle", value_name = "PATH", num_args = 1..)]
    pub bundles: Option<Vec<PathBuf>>,
    /// Install after signing
    #[arg(long, conflicts_with = "output")]
    pub install: bool,
    #[command(flatten)]
    pub device: DeviceTargetArgs,
    /// Output path for a signed .ipa export
    #[arg(long, short, value_name = "OUTPUT")]
    pub output: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum PostSignAction {
    StayInPlace,
    Install,
    Export(PathBuf),
}

/// True when the user named a concrete install target (UDID or "this Mac").
#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
fn sign_args_has_explicit_device_target(args: &SignArgs) -> bool {
    args.device.udid.is_some() || args.device.mac
}

#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
fn sign_args_has_explicit_device_target(args: &SignArgs) -> bool {
    args.device.udid.is_some()
}

pub async fn execute(args: SignArgs) -> Result<()> {
    let input_is_bundle = args.package.is_dir();
    let action = resolve_post_sign_action(&args)?;
    let sign_mode = resolve_sign_mode(&args)?;
    let device = match action {
        PostSignAction::Install => Some(select_target(&args.device).await?),
        _ => None,
    };

    let mut options = build_signer_options(&args, input_is_bundle)?;
    options.mode = sign_mode;
    options.install_mode = match action {
        PostSignAction::Export(_) => SignerInstallMode::Export,
        _ => SignerInstallMode::Install,
    };

    let (bundle, package) = load_input(&args.package, input_is_bundle)?;
    let (mut signer, account_team) = build_signer(&options, &args, device.as_ref()).await?;

    if let Some(provision_path) = args.provisioning_file.as_ref() {
        signer
            .provisioning_files
            .push(MobileProvision::load_with_path(provision_path)?);
    }

    if let Some((account, session, team_id)) = account_team.as_ref() {
        signer
            .modify_bundle(&bundle, &Some(team_id.clone()))
            .await?;

        if let Some(dev) = device.as_ref() {
            if !dev.is_mac {
                session
                    .qh_ensure_device(team_id, &dev.name, &dev.udid)
                    .await?;
            }
        }

        signer
            .register_bundle(&bundle, session, team_id, false)
            .await?;
        signer.sign_bundle(&bundle).await?;

        if options.refresh {
            let mut store = load_store().await?;
            save_refresh_registration(&mut store, &bundle, &device, account).await?;
        }
    } else if options.mode != SignerMode::None {
        signer.modify_bundle(&bundle, &None).await?;
        signer.sign_bundle(&bundle).await?;
    }

    match action {
        PostSignAction::StayInPlace => {
            log::info!(
                "Signed bundle in place at {}",
                bundle.bundle_dir().display()
            );
        }
        PostSignAction::Install => {
            let device = device.ok_or_else(|| anyhow!("No device selected for install"))?;
            install_signed_bundle(&bundle, &device).await?;
            maybe_install_pairing(&bundle, &signer.options, &device).await?;
        }
        PostSignAction::Export(output) => {
            let package = package
                .as_ref()
                .ok_or_else(|| anyhow!("Export is only supported for .ipa input"))?;
            let archived_path = package.get_archive_based_on_path(&args.package)?;
            tokio::fs::copy(&archived_path, &output).await?;
            log::info!("Saved signed package to {}", output.display());
        }
    }

    if let Some(pkg) = package {
        if std::env::var("PLUME_DELETE_AFTER_FINISHED").is_err() {
            pkg.remove_package_stage();
        }
    }

    Ok(())
}
