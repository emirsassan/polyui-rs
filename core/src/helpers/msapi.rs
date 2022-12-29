use crate::{entities::launcher::msapi as inner, state::State};
use futures::prelude::*;
use tokio::sync::oneshot;

pub use inner::Credentials;

#[tracing::instrument]
pub(crate) async fn authenticate(
    browser_url: oneshot::Sender<url::Url>,
) -> crate::error::Result<Credentials> {
    let mut flow = inner::PolyAuthFlow::new().await?;
    let state = State::get().await?;
    let mut users = state.users.write().await;

    let url = flow.prepare_login_url().await?;
    browser_url.send(url).map_err(|url| {
        crate::error::CoreErrors::OtherError(format!(
            "Error sending browser url to parent: {url}"
        ))
    })?;

    let credentials = flow.extract_credentials().await?;
    users.insert(&credentials)?;

    if state.settings.read().await.default_user.is_none() {
        let mut settings = state.settings.write().await;
        settings.default_user = Some(credentials.id);
    }

    Ok(Credentials::from(credentials))
}

#[tracing::instrument]
pub async fn refresh(
    user: uuid::Uuid,
    update_name: bool,
) -> crate::error::Result<Credentials> {
    let state = State::get().await?;
    let mut users = state.users.write().await;

    futures::future::ready(users.get(user)?.ok_or_else(|| {
        crate::error::CoreErrors::OtherError(format!(
            "Tried to refresh nonexistant user with ID {user}"
        ))
        .as_error()
    }))
    .and_then(|mut credentials| async move {
        if chrono::offset::Utc::now() > credentials.expires {
            inner::refresh_credentials(&mut credentials).await?;
            if update_name {
                inner::refresh_username(&mut credentials).await?;
            }
        }
        users.insert(&credentials)?;
        Ok(credentials)
    })
    .await
}

#[tracing::instrument]
pub async fn remove_user(user: uuid::Uuid) -> crate::error::Result<()> {
    let state = State::get().await?;
    let mut users = state.users.write().await;

    if state.settings.read().await.default_user == Some(user) {
        let mut settings = state.settings.write().await;
        settings.default_user = users
            .0
            .first()?
            .map(|it| uuid::Uuid::from_slice(&it.0))
            .transpose()?;
    }

    users.remove(user)?;
    Ok(())
}

#[tracing::instrument]
pub async fn has_user(user: uuid::Uuid) -> crate::error::Result<bool> {
    let state = State::get().await?;
    let users = state.users.read().await;

    Ok(users.contains(user)?)
}

#[tracing::instrument]
pub async fn users() -> crate::error::Result<Box<[Credentials]>> {
    let state = State::get().await?;
    let users = state.users.read().await;
    users.iter().collect()
}