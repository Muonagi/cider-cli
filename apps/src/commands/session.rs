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

    let sp = crate::ui::spinner(format!("Restoring session for {}...", account.email()));

    let session = DeveloperSession::new(
        account.adsid().clone(),
        account.xcode_gs_token().clone(),
        anisette_config,
    )
    .await?;

    crate::ui::finish_spinner(&sp, format!("Session restored for {}", account.email()));

    Ok(session)
}

pub fn available_teams(session: &DeveloperSession) -> Vec<TeamChoice> {
    session
        .cached_teams()
        .iter()
        .map(|team| TeamChoice {
            name: team.name.clone(),
            id: team.team_id.clone(),
        })
        .collect()
}

pub fn resolve_team_id(
    session: &DeveloperSession,
    account: &GsaAccount,
    requested_team: Option<&str>,
    prompt_if_needed: bool,
) -> Result<String> {
    if let Some(team_id) = requested_team {
        let teams = available_teams(session);
        if teams.iter().any(|team| team.id == team_id) {
            return Ok(team_id.to_string());
        }
        return Err(anyhow!(
            "Team '{}' was not found for account {}",
            team_id,
            account.email()
        ));
    }

    // Fast path: use stored team_id without re-querying
    if !prompt_if_needed && !account.team_id().is_empty() {
        return Ok(account.team_id().clone());
    }

    let teams = available_teams(session);

    if teams.is_empty() {
        return Err(anyhow!(
            "No teams available for account {}",
            account.email()
        ));
    }

    if !prompt_if_needed {
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
