mod refresh;
mod requestor;
mod response;

pub use self::{
	refresh::Refresh,
	requestor::{Request, Requestor},
	response::Response,
};

fn http_client() -> &'static reqwest::Client {
	use once_cell::sync::Lazy;
	static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| reqwest::Client::new());
	&HTTP_CLIENT
}
