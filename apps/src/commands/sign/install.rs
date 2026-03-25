use anyhow::{Result, anyhow};

use plume_core::MobileProvision;
use plume_store::{AccountStore, RefreshApp, RefreshDevice};
use plume_utils::{Bundle, PlistInfoTrait, SignerOptions};

use crate::{
    commands::{refresh::scheduled_refresh_from_provision, session::get_settings_path},
    get_data_path,
};

pub(super) async fn install_signed_bundle(
    bundle: &Bundle,
    device: &plume_utils::Device,
) -> Result<()> {
    log::info!("Installing to device: {}", device.name);

    if device.is_mac {
        plume_utils::install_app_mac(bundle.bundle_dir()).await?;
    } else {
        device
            .install_app(bundle.bundle_dir(), |progress| async move {
                log::info!("Installation progress: {}%", progress);
            })
            .await?;
    }

    Ok(())
}

pub(super) async fn maybe_install_pairing(
    bundle: &Bundle,
    options: &SignerOptions,
    device: &plume_utils::Device,
) -> Result<()> {
    if device.is_mac || !options.app.supports_pairing_file() {
        return Ok(());
    }

    if let (Some(bundle_id), Some(pairing_path)) = (
        bundle.get_bundle_identifier(),
        options.app.pairing_file_path(),
    ) {
        let _ = device
            .install_pairing_record(&bundle_id, pairing_path)
            .await;
    }

    Ok(())
}

pub(super) async fn load_store() -> Result<AccountStore> {
    Ok(AccountStore::load(&Some(get_settings_path())).await?)
}

pub(super) async fn save_refresh_registration(
    store: &mut AccountStore,
    bundle: &Bundle,
    device: &Option<plume_utils::Device>,
    account: &plume_store::GsaAccount,
) -> Result<()> {
    let device = device
        .as_ref()
        .ok_or_else(|| anyhow!("--refresh requires an install target device"))?;

    let path = get_data_path().join("refresh_store");
    tokio::fs::create_dir_all(&path).await?;

    let original_name = bundle
        .bundle_dir()
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| anyhow!("Bundle name is missing"))?;
    let dest_name = format!(
        "{}-{}.app",
        original_name.trim_end_matches(".app"),
        uuid::Uuid::new_v4()
    );
    let dest_path = path.join(dest_name);

    plume_utils::copy_dir_recursively(bundle.bundle_dir(), &dest_path).await?;

    let provision = MobileProvision::load_with_path(&dest_path.join("embedded.mobileprovision"))?;
    let scheduled_refresh = scheduled_refresh_from_provision(&provision);

    let refresh_app = RefreshApp {
        name: bundle.get_name(),
        bundle_id: bundle.get_bundle_identifier(),
        path: dest_path,
        scheduled_refresh,
    };

    let mut refresh_device = store
        .get_refresh_device(&device.udid)
        .cloned()
        .unwrap_or_else(|| RefreshDevice {
            udid: device.udid.clone(),
            name: device.name.clone(),
            account: account.email().clone(),
            apps: Vec::new(),
            is_mac: device.is_mac,
        });

    if let Some(existing_app) = refresh_device
        .apps
        .iter_mut()
        .find(|app| app.bundle_id == refresh_app.bundle_id)
    {
        *existing_app = refresh_app;
    } else {
        refresh_device.apps.push(refresh_app);
    }

    store.add_or_update_refresh_device(refresh_device).await?;
    Ok(())
}
