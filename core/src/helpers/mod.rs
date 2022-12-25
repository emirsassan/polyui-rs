use std::{io, path::PathBuf, sync::Arc};
use thiserror::Error;

use crate::{entities::microsoft::XboxLiveSTSAuthorizeResponse, NodeContext};

use self::msauth::MSAuthError;

mod msauth;

pub struct HelpersManager {
	node_context: NodeContext,
}

#[derive(Error, Debug)]
pub enum HelpersManagerError {
	#[error("error saving or loading the config from the filesystem")]
	IO(#[from] io::Error),
	#[error("error serializing or deserializing the JSON in the config file")]
	Json(#[from] serde_json::Error),
	#[error("Database error: {0}")]
	Database(#[from] prisma_client_rust::QueryError),
	#[error("Library not found error")]
	LibraryNotFound,
	#[error("error migrating the config file")]
	Migration(String),
	#[error("failed to parse uuid")]
	Uuid(#[from] uuid::Error),
	#[error("error opening database as the path contains non-UTF-8 characters")]
	InvalidDatabasePath(PathBuf),
	#[error("error occured during ms auth")]
	MSAuthHelper(#[from] MSAuthError),
}

impl From<HelpersManagerError> for rspc::Error {
	fn from(error: HelpersManagerError) -> Self {
		rspc::Error::with_cause(
			rspc::ErrorCode::InternalServerError,
			error.to_string(),
			error,
		)
	}
}

impl HelpersManager {
	pub(crate) async fn new(node_context: NodeContext) -> Arc<Self> {
		return Arc::new(Self { node_context });
	}

	pub(crate) async fn run_ms_auth(
		&self,
	) -> Result<XboxLiveSTSAuthorizeResponse, HelpersManagerError> {
		let token = msauth::login_flow()
			.await
			.map_err(|err| HelpersManagerError::MSAuthHelper(err));
		return token;
	}
}
