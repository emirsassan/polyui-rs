use crate::{prisma::PrismaClient, config::BINCODE_CONFIG};

use super::settings::{Hooks, MemorySettings, WindowSize};
use daedalus::modded::LoaderVersion;
use futures::prelude::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use tokio::fs;

const PROFILE_JSON_PATH: &str = "profile.json";
const PROFILE_SUBTREE: &[u8] = b"profiles";

pub(crate) struct Profiles(pub HashMap<PathBuf, Option<Profile>>);

pub const CURRENT_FORMAT_VERSION: u32 = 1;
pub const SUPPORTED_ICON_FORMATS: &[&'static str] = &[
    "bmp", "gif", "jpeg", "jpg", "jpe", "png", "svg", "svgz", "webp", "rgb",
    "mp4",
];

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Profile {
    #[serde(skip)]
    pub path: PathBuf,
    pub metadata: ProfileMetadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java: Option<JavaSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<MemorySettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<WindowSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hooks: Option<Hooks>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProfileMetadata {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<PathBuf>,
    pub game_version: String,
    #[serde(default)]
    pub loader: ModLoader,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loader_version: Option<LoaderVersion>,
    pub format_version: u32,
}

// TODO: Quilt?
#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ModLoader {
    Vanilla,
    Forge,
    Fabric,
}

impl Default for ModLoader {
    fn default() -> Self {
        ModLoader::Vanilla
    }
}

impl std::fmt::Display for ModLoader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            &Self::Vanilla => "Vanilla",
            &Self::Forge => "Forge",
            &Self::Fabric => "Fabric",
        })
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JavaSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install: Option<PathBuf>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_arguments: Option<Vec<String>>,
}

impl Profile {
    #[tracing::instrument]
    pub async fn new(
        name: String,
        version: String,
        path: PathBuf,
    ) -> crate::error::Result<Self> {
        if name.trim().is_empty() {
            return Err(crate::error::CoreErrors::InputError(String::from(
                "Empty name for instance!",
            ))
            .into());
        }

        Ok(Self {
            path: path.canonicalize()?,
            metadata: ProfileMetadata {
                name,
                icon: None,
                game_version: version,
                loader: ModLoader::Vanilla,
                loader_version: None,
                format_version: CURRENT_FORMAT_VERSION,
            },
            java: None,
            memory: None,
            resolution: None,
            hooks: None,
        })
    }

    #[tracing::instrument]
    pub fn with_name(&mut self, name: String) -> &mut Self {
        self.metadata.name = name;
        self
    }

    #[tracing::instrument]
    pub async fn with_icon<'a>(
        &'a mut self,
        icon: &'a Path,
    ) -> crate::error::Result<&'a mut Self> {
        let ext = icon
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("");

        if SUPPORTED_ICON_FORMATS.contains(&ext) {
            let file_name = format!("icon.{ext}");
            fs::copy(icon, &self.path.join(&file_name)).await?;
            self.metadata.icon =
                Some(Path::new(&format!("./{file_name}")).to_owned());

            Ok(self)
        } else {
            Err(crate::error::CoreErrors::InputError(format!(
                "Unsupported image type: {ext}"
            ))
            .into())
        }
    }

    #[tracing::instrument]
    pub fn with_game_version(&mut self, version: String) -> &mut Self {
        self.metadata.game_version = version;
        self
    }

    #[tracing::instrument]
    pub fn with_loader(
        &mut self,
        loader: ModLoader,
        version: Option<LoaderVersion>,
    ) -> &mut Self {
        self.metadata.loader = loader;
        self.metadata.loader_version = version;
        self
    }

    #[tracing::instrument]
    pub fn with_java_settings(
        &mut self,
        settings: Option<JavaSettings>,
    ) -> &mut Self {
        self.java = settings;
        self
    }

    #[tracing::instrument]
    pub fn with_memory(
        &mut self,
        settings: Option<MemorySettings>,
    ) -> &mut Self {
        self.memory = settings;
        self
    }

    #[tracing::instrument]
    pub fn with_resolution(
        &mut self,
        resolution: Option<WindowSize>,
    ) -> &mut Self {
        self.resolution = resolution;
        self
    }

    #[tracing::instrument]
    pub fn with_hooks(&mut self, hooks: Option<Hooks>) -> &mut Self {
        self.hooks = hooks;
        self
    }
}

impl Profiles {
    pub async fn init(db: PrismaClient) -> crate::error::Result<Self> {
        let profiles: HashMap<PathBuf, Option<Profile>> = HashMap::new();

        Ok(Self(profiles))
    }
}