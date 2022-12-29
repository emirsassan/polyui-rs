use tracing_error::InstrumentError;

#[derive(thiserror::Error, Debug)]
pub enum CoreErrors {
    #[error("Filesystem error: {0}")]
    FSError(String),

    #[error("Serialization error (JSON): {0}")]
    JSONError(#[from] serde_json::Error),

    #[error("Error parsing UUID: {0}")]
    UUIDError(#[from] uuid::Error),

    #[error("Error parsing URL: {0}")]
    URLError(#[from] url::ParseError),

    #[error("Database query error: {0}")]
    DBQuery(#[from] prisma_client_rust::QueryError),

    #[error("Database client error: {0}")]
    DBClient(#[from] prisma_client_rust::NewClientError),

    #[error("Unable to read {0} from any source")]
    NoValueFor(String),

    #[error("Metadata error: {0}")]
    MetadataError(#[from] daedalus::Error),

    #[error("MSAPI authentication error: {0}")]
    MSApiError(String),

    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("Error launching Minecraft: {0}")]
    LauncherError(String),

    #[error("Error fetching URL: {0}")]
    FetchError(#[from] reqwest::Error),

    #[error("Websocket error: {0}")]
    WSError(#[from] async_tungstenite::tungstenite::Error),

    #[error("Websocket closed before {0} could be received!")]
    WSClosedError(String),

    #[error("Incorrect Sha1 hash for download: {0} != {1}")]
    HashError(String, String),

    #[error("Paths stored in the database need to be valid UTF-8: {0}")]
    UTFError(std::path::PathBuf),

    #[error("Invalid input: {0}")]
    InputError(String),

    #[error(
        "Tried to access unloaded profile {0}, loading it probably failed"
    )]
    UnloadedProfileError(String),

    #[error("Profile {0} is not managed by PolyUI!")]
    UnmanagedProfileError(String),

    #[error("Error: {0}")]
    OtherError(String),
}

#[derive(Debug)]
pub struct Error {
    source: tracing_error::TracedError<CoreErrors>,
}

impl From<Error> for rspc::Error {
    fn from(err: Error) -> Self {
        rspc::Error::new(rspc::ErrorCode::InternalServerError, err.to_string().into())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.source()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{}", self.source)
    }
}

impl<E: Into<CoreErrors>> From<E> for Error {
    fn from(source: E) -> Self {
        Self {
            source: Into::<CoreErrors>::into(source).in_current_span(),
        }
    }
}

impl CoreErrors {
    pub fn as_error(self) -> Error {
        self.into()
    }
}

pub type Result<T> = core::result::Result<T, Error>;
