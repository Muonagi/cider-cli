use std::path::PathBuf;

use anyhow::{Result, anyhow};
use dialoguer::Select;

use plume_core::{AnisetteConfiguration, developer::DeveloperSession};
use plume_store::{AccountStore, GsaAccount};

use crate::get_data_path;

#[derive(Debug, Clone)]
pub struct TeamChoice {
    pub name: String,
    pub id: String,
}

impl std::fmt::Display for TeamChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.id)
    }
}

pub(crate) fn get_settings_path() -> PathBuf {
    get_data_path().join("accounts.json")
}

pub async fn load_account_store() -> Result<AccountStore> {
    Ok(AccountStore::load(&Some(get_settings_path())).await?)
}

pub async fn get_selected_account() -> Result<GsaAccount> {
    let settings = load_account_store().await?;
    settings.selected_account().cloned().ok_or_else(|| {
        anyhow!("No account selected. Please login first using 'impactor account login'")
    })
}

pub async fn get_account_by_email(email: &str) -> Result<GsaAccount> {
    let settings = load_account_store().await?;
    settings.get_account(email).cloned().ok_or_else(|| {
        anyhow!(
            "Account '{email}' not found. Use 'impactor account list' to see available accounts."
        )
    })
}

pub async fn get_selected_or_named_account(email: Option<&str>) -> Result<GsaAccount> {
    match email {
        Some(email) => get_account_by_email(email).await,
        None => get_selected_account().await,
    }
}

pub async fn create_session(account: &GsaAccount) -> Result<DeveloperSession> {
    let anisette_config = AnisetteConfiguration::default().set_configuration_path(get_data_path());

    log::info!("Restoring session for {}...", account.email());

    Ok(DeveloperSession::new(
        account.adsid().clone(),
        account.xcode_gs_token().clone(),
        anisette_config,
    )
    .await?)
}

pub async fn available_teams(account: &GsaAccount) -> Result<Vec<TeamChoice>> {
    let session = create_session(account).await?;
    let teams = session.qh_list_teams().await?.teams;
    Ok(teams
        .into_iter()
        .map(|team| TeamChoice {
            name: team.name,
            id: team.team_id,
        })
        .collect())
}

pub async fn resolve_team_id(
    account: &GsaAccount,
    requested_team: Option<&str>,
    prompt_if_needed: bool,
) -> Result<String> {
    let teams = available_teams(account).await?;

    if teams.is_empty() {
        return Err(anyhow!(
            "No teams available for account {}",
            account.email()
        ));
    }

    if let Some(team_id) = requested_team {
        if teams.iter().any(|team| team.id == team_id) {
            return Ok(team_id.to_string());
        }

        return Err(anyhow!(
            "Team '{}' was not found for account {}",
            team_id,
            account.email()
        ));
    }

    if !prompt_if_needed {
        if !account.team_id().is_empty() && teams.iter().any(|team| team.id == *account.team_id()) {
            return Ok(account.team_id().clone());
        }

        return Ok(teams[0].id.clone());
    }

    if teams.len() == 1 {
        return Ok(teams[0].id.clone());
    }

    let default_index = teams
        .iter()
        .position(|team| team.id == *account.team_id())
        .unwrap_or(0);

    let selection = Select::new()
        .with_prompt("Select a team")
        .items(&teams)
        .default(default_index)
        .interact()?;

    Ok(teams[selection].id.clone())
}
