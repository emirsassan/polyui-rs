use std::sync::Arc;
use tokio::sync::{Mutex, OnceCell, RwLock, Semaphore};

pub(crate) mod dirs;
pub use self::dirs::*;

pub(crate) mod metadata;
pub use self::metadata::*;

pub(crate) mod profiles;
pub use self::profiles::*;

pub(crate) mod settings;
pub use self::settings::*;

mod users;
pub use self::users::*;

static LAUNCHER_STATE: OnceCell<Arc<State>> = OnceCell::const_new();
pub struct State {
    pub(self) database: crate::prisma::PrismaClient,
    /// Information on the location of files used in the launcher
    pub directories: DirectoryInfo,
    /// Semaphore used to limit concurrent I/O and avoid errors
    pub io_semaphore: Semaphore,
    /// Launcher metadata
    pub metadata: Metadata,
    // TODO: settings API
    /// Launcher configuration
    pub settings: RwLock<Settings>,
    /// Launcher profile metadata
    pub(crate) profiles: RwLock<Profiles>,
    /// Launcher user account info
    pub(crate) users: RwLock<Users>,
}

impl State {
    pub async fn get() {
        
    }
}