pub use crate::state::{JavaSettings, Profile, State};
use daedalus as d;
use std::{future::Future, path::{Path, PathBuf}};
use tokio::process::{Child, Command};

/// Add a profile to the in-memory state
#[tracing::instrument]
pub async fn add(profile: Profile) -> crate::error::Result<()> {
    let state = State::get().await?;
    let mut profiles = state.profiles.write().await;
    profiles.insert(profile)?;

    Ok(())
}

/// Add a path as a profile in-memory
#[tracing::instrument]
pub async fn add_path(path: &Path) -> crate::error::Result<()> {
    let state = State::get().await?;
    let mut profiles = state.profiles.write().await;
    profiles.insert_from(path).await?;

    Ok(())
}

/// Remove a profile
#[tracing::instrument]
pub async fn remove(path: &Path) -> crate::error::Result<()> {
    let state = State::get().await?;
    let mut profiles = state.profiles.write().await;
    profiles.remove(path)?;

    Ok(())
}

/// Get a profile by path,
#[tracing::instrument]
pub async fn get(path: &Path) -> crate::error::Result<Option<Profile>> {
    let state = State::get().await?;
    let profiles = state.profiles.read().await;

    profiles.0.get(path).map_or(Ok(None), |prof| match prof {
        Some(prof) => Ok(Some(prof.clone())),
        None => Err(crate::error::CoreErrors::UnloadedProfileError(
            path.display().to_string(),
        )
        .as_error()),
    })
}

/// Check if a profile is already managed by PolyUI
#[tracing::instrument]
pub async fn is_managed(profile: &Path) -> crate::error::Result<bool> {
    let state = State::get().await?;
    let profiles = state.profiles.read().await;
    Ok(profiles.0.contains_key(profile))
}

/// Check if a profile is loaded
#[tracing::instrument]
pub async fn is_loaded(profile: &Path) -> crate::error::Result<bool> {
    let state = State::get().await?;
    let profiles = state.profiles.read().await;
    Ok(profiles
        .0
        .get(profile)
        .map(Option::as_ref)
        .flatten()
        .is_some())
}

/// Edit a profile using a given asynchronous closure
pub async fn edit<Fut>(
    path: &Path,
    action: impl Fn(&mut Profile) -> Fut,
) -> crate::error::Result<()>
where
    Fut: Future<Output = crate::error::Result<()>>,
{
    let state = State::get().await?;
    let mut profiles = state.profiles.write().await;

    match profiles.0.get_mut(path) {
        Some(&mut Some(ref mut profile)) => action(profile).await,
        Some(&mut None) => Err(crate::error::CoreErrors::UnloadedProfileError(
            path.display().to_string(),
        )
        .as_error()),
        None => Err(crate::error::CoreErrors::UnmanagedProfileError(
            path.display().to_string(),
        )
        .as_error()),
    }
}

/// Get a copy of the profile set
#[tracing::instrument]
pub async fn list(
) -> crate::error::Result<std::collections::HashMap<PathBuf, Option<Profile>>> {
    let state = State::get().await?;
    let profiles = state.profiles.read().await;
    Ok(profiles.0.clone())
}

/// Run Minecraft using a profile
#[tracing::instrument(skip_all)]
pub async fn run(
    path: &Path,
    credentials: &crate::api::prelude::Credentials,
) -> crate::error::Result<Child> {
    let state = State::get().await.unwrap();
    let settings = state.settings.read().await;
    let profile = get(path).await?.ok_or_else(|| {
        crate::error::CoreErrors::OtherError(format!(
            "Tried to run a nonexistent or unloaded profile at path {}!",
            path.display()
        ))
    })?;

    let version = state
        .metadata
        .minecraft
        .versions
        .iter()
        .find(|it| it.id == profile.metadata.game_version.as_ref())
        .ok_or_else(|| {
            crate::error::CoreErrors::LauncherError(format!(
                "Invalid or unknown Minecraft version: {}",
                profile.metadata.game_version
            ))
        })?;
    let version_info = d::minecraft::fetch_version_info(version).await?;

    let ref pre_launch_hooks =
        profile.hooks.as_ref().unwrap_or(&settings.hooks).pre_launch;
    for hook in pre_launch_hooks.iter() {
        let mut cmd = hook.split(' ');
        let result = Command::new(cmd.next().unwrap())
            .args(&cmd.collect::<Vec<&str>>())
            .current_dir(path)
            .spawn()?
            .wait()
            .await?;

        if !result.success() {
            return Err(crate::error::CoreErrors::LauncherError(format!(
                "Non-zero exit code for pre-launch hook: {}",
                result.code().unwrap_or(-1)
            ))
            .as_error());
        }
    }

    let java_install = match profile.java {
        Some(JavaSettings {
            install: Some(ref install),
            ..
        }) => install,
        _ => if version_info
            .java_version
            .as_ref()
            .filter(|it| it.major_version >= 16)
            .is_some()
        {
            settings.java_17_path.as_ref()
        } else {
            settings.java_8_path.as_ref()
        }
        .ok_or_else(|| {
            crate::error::CoreErrors::LauncherError(format!(
                "No Java installed for version {}",
                version_info.java_version.map_or(8, |it| it.major_version),
            ))
        })?,
    };

    if !java_install.exists() {
        return Err(crate::error::CoreErrors::LauncherError(format!(
            "Could not find Java install: {}",
            java_install.display()
        ))
        .as_error());
    }

    let ref java_args = profile
        .java
        .as_ref()
        .and_then(|it| it.extra_arguments.as_ref())
        .unwrap_or(&settings.custom_java_args);

    let wrapper = profile
        .hooks
        .as_ref()
        .map_or(&settings.hooks.wrapper, |it| &it.wrapper);

    let ref memory = profile.memory.unwrap_or(settings.memory);
    let ref resolution = profile.resolution.unwrap_or(settings.game_resolution);

    crate::entities::launcher::launch_minecraft(
        &profile.metadata.game_version,
        &profile.metadata.loader_version,
        &profile.path,
        &java_install,
        &java_args,
        &wrapper,
        memory,
        resolution,
        credentials,
    )
    .await
}

#[tracing::instrument]
pub async fn kill(running: &mut Child) -> crate::error::Result<()> {
    running.kill().await?;
    wait_for(running).await
}

#[tracing::instrument]
pub async fn wait_for(running: &mut Child) -> crate::error::Result<()> {
    let result = running.wait().await.map_err(|err| {
        crate::error::CoreErrors::LauncherError(format!(
            "Error running minecraft: {err}"
        ))
    })?;

    match result.success() {
        false => Err(crate::error::CoreErrors::LauncherError(format!(
            "Minecraft exited with non-zero code {}",
            result.code().unwrap_or(-1)
        ))
        .as_error()),
        true => Ok(()),
    }
}