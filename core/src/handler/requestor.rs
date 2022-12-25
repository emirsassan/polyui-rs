use super::{http_client, Refresh, Response};

#[async_trait::async_trait]
pub trait Request: Sized + serde::ser::Serialize {
	type Client;
	type Response;
	type Rejection: std::fmt::Debug;

	async fn request(self, client: &Self::Client) -> Result<Self::Response, Self::Rejection>;
}

#[derive(Debug)]
pub struct Requestor<T>(pub(super) T);

impl<T: Request<Client = reqwest::Client>> Requestor<T> {
	pub fn new(order: T) -> Self {
		Self(order)
	}

	pub async fn execute(self) -> Result<Response<T::Response>, T::Rejection> {
		match self.0.request(http_client()).await {
			Ok(res) => Ok(Response::new(res)),
			Err(error) => {
				tracing::error!("request was not processed correctly. {:?}", error);
				Err(error)
			}
		}
	}
}
