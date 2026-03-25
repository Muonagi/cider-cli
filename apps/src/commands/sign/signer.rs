use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
use dialoguer::{Input, Select};

use plume_core::CertificateIdentity;
use plume_utils::{Bundle, Package, PlistInfoTrait, Signer, SignerApp, SignerMode, SignerOptions};

use crate::{
    commands::session::{create_session, get_selected_account, resolve_team_id},
    get_data_path,
};

use super::SignArgs;

pub(super) fn build_signer_options(
    args: &SignArgs,
    input_is_bundle: bool,
) -> Result<SignerOptions> {
    let mut options = if input_is_bundle {
        let bundle = Bundle::new(&args.package)?;
        SignerOptions::new_for_app(SignerApp::from_bundle_identifier_or_name(
            bundle.get_bundle_identifier().as_deref(),
            bundle.get_name().as_deref(),
        ))
    } else {
        let package = Package::new(args.package.clone())?;
        let mut options = SignerOptions::default();
        package.load_into_signer_options(&mut options);
        package.remove_package_stage();
        options
    };

    options.custom_identifier = args.bundle_identifier.clone();
    options.custom_name = args.name.clone();
    options.custom_version = args.version.clone();
    options.custom_icon = args.custom_icon.clone();
    options.custom_entitlements = args.custom_entitlements.clone();
    options.features.support_minimum_os_version = args.support_minimum_os_version;
    options.features.support_file_sharing = args.support_file_sharing;
    options.features.support_ipad_fullscreen = args.support_ipad_fullscreen;
    options.features.support_game_mode = args.support_game_mode;
    options.features.support_pro_motion = args.support_pro_motion;
    options.features.support_liquid_glass = args.support_liquid_glass;
    options.features.support_ellekit = args.support_ellekit;
    options.embedding.single_profile = args.single_profile;
    options.refresh = args.refresh;
    options.tweaks = merge_injection_paths(args.tweaks.clone(), args.bundles.clone());

    Ok(options)
}

pub(super) async fn build_signer(
    options: &SignerOptions,
    args: &SignArgs,
    device: Option<&plume_utils::Device>,
) -> Result<(
    Signer,
    Option<(
        plume_store::GsaAccount,
        plume_core::developer::DeveloperSession,
        String,
    )>,
)> {
    if let Some(pem_files) = args.pem_files.as_ref() {
        let identity = CertificateIdentity::new_with_paths(Some(pem_files.clone())).await?;
        return Ok((Signer::new(Some(identity), options.clone()), None));
    }

    if options.mode == SignerMode::Pem {
        let account = get_selected_account().await?;
        let session = create_session(&account).await?;
        let team_id = resolve_team_id(&session, &account, None, false)?;
        let identity =
            CertificateIdentity::new_with_session(&session, get_data_path(), None, &team_id, false)
                .await?;

        return Ok((
            Signer::new(Some(identity), options.clone()),
            Some((account, session, team_id)),
        ));
    }

    if options.mode == SignerMode::Adhoc
        && !args.package.is_dir()
        && args.output.is_none()
        && device.is_none()
    {
        return Err(anyhow!(
            "Ad-hoc signing of an .ipa needs either --install or --output."
        ));
    }

    Ok((Signer::new(None, options.clone()), None))
}

pub(super) fn load_input(
    package_path: &Path,
    input_is_bundle: bool,
) -> Result<(Bundle, Option<Package>)> {
    if input_is_bundle {
        let bundle = Bundle::new(package_path)?;
        return Ok((bundle, None));
    }

    let package = Package::new(package_path.to_path_buf())?;
    let bundle = package.get_package_bundle()?;
    Ok((bundle, Some(package)))
}

pub(super) fn resolve_sign_mode(args: &SignArgs) -> Result<SignerMode> {
    if args.no_modify {
        return Ok(SignerMode::None);
    }

    let selected = [args.adhoc, args.apple_id, args.pem_files.as_ref().is_some()]
        .into_iter()
        .filter(|value| *value)
        .count();

    if selected > 1 {
        return Err(anyhow!(
            "Choose only one signing mode: --apple-id, --adhoc, --no-modify, or --pem."
        ));
    }

    if args.adhoc {
        return Ok(SignerMode::Adhoc);
    }

    if args.apple_id || args.pem_files.is_some() {
        return Ok(SignerMode::Pem);
    }

    let choices = ["Apple ID", "Adhoc", "No Modify"];
    let selection = Select::new()
        .with_prompt("Select a signing mode")
        .items(&choices)
        .default(0)
        .interact()?;

    Ok(match selection {
        0 => SignerMode::Pem,
        1 => SignerMode::Adhoc,
        _ => SignerMode::None,
    })
}

pub(super) fn resolve_post_sign_action(args: &SignArgs) -> Result<super::PostSignAction> {
    use super::PostSignAction;

    if args.package.is_dir() {
        if let Some(output) = args.output.as_ref() {
            return Err(anyhow!(
                "Exporting a signed .app bundle is not supported (requested {}).",
                output.display()
            ));
        }

        if args.install || super::sign_args_has_explicit_device_target(args) {
            return Ok(PostSignAction::Install);
        }

        return Ok(PostSignAction::StayInPlace);
    }

    if let Some(output) = args.output.as_ref() {
        return Ok(PostSignAction::Export(output.clone()));
    }

    if args.install || super::sign_args_has_explicit_device_target(args) {
        return Ok(PostSignAction::Install);
    }

    let options = ["Install to device", "Export signed .ipa"];
    let selection = Select::new()
        .with_prompt("What do you want to do after signing?")
        .items(&options)
        .default(0)
        .interact()?;

    if selection == 0 {
        Ok(PostSignAction::Install)
    } else {
        let default = default_export_path(&args.package);
        let output: String = Input::new()
            .with_prompt("Output path")
            .default(default.to_string_lossy().to_string())
            .interact_text()?;
        Ok(PostSignAction::Export(PathBuf::from(output)))
    }
}

fn default_export_path(input: &Path) -> PathBuf {
    let parent = input.parent().unwrap_or_else(|| Path::new("."));
    let stem = input
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("signed");
    parent.join(format!("{stem}-signed.ipa"))
}

fn merge_injection_paths(
    tweaks: Option<Vec<PathBuf>>,
    bundles: Option<Vec<PathBuf>>,
) -> Option<Vec<PathBuf>> {
    let mut combined = Vec::new();

    if let Some(mut tweaks) = tweaks {
        combined.append(&mut tweaks);
    }

    if let Some(mut bundles) = bundles {
        combined.append(&mut bundles);
    }

    (!combined.is_empty()).then_some(combined)
}
