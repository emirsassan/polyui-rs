use reqwest::StatusCode;
use std::time::Duration;
use thiserror::Error;
use tokio::task::JoinHandle;

use crate::{
	entities::microsoft::{
		FlowSteppable, MSADeviceAuthenticateReponse, MSADeviceAuthenticateRequest,
		MSADeviceFlowError, MSADeviceFlowErrorResponse, MSADeviceVerifingRequest,
		MSADeviceVerifingResponse, XboxLiveSTSAuthorizeRequest, XboxLiveSTSAuthorizeResponse,
		XboxLiveUserAuthenticateRequest, XboxLiveUserAuthenticateResponse,
	},
	handler::Requestor,
};

#[derive(Error, Debug)]
pub enum MSAuthError {
	#[error("error during MSA device flow polling {0}")]
	PollingError(#[from] MSADeviceFlowPollingError),
	#[error("error during XSTS flow authentication {0}")]
	XSTSError(#[from] reqwest::Error),
}

pub async fn login_flow() -> Result<XboxLiveSTSAuthorizeResponse, MSAuthError> {
	let res = Requestor::new(MSADeviceVerifingRequest)
		.execute()
		.await
		.unwrap()
		.map_future(|res| res.json::<MSADeviceVerifingResponse>())
		.await
		.unwrap();
	let interval = res.interval();
	let expires_in = res.expires_in();

	let polling: JoinHandle<Result<_, MSADeviceFlowPollingError>> =
		tokio::task::spawn(async move {
			let mut retry = 0;
			loop {
				tokio::time::sleep(Duration::from_secs(interval as u64)).await;

				let req = Requestor::new(MSADeviceAuthenticateRequest::new(res.device_code()))
					.execute()
					.await
					.map_err(|_| MSADeviceFlowPollingError::Unknown)?;

				if req.peeking(|inner| inner.status()) != StatusCode::OK {
					match req
						.map_future(|res| res.json::<MSADeviceFlowErrorResponse>())
						.await
						.map_err(MSADeviceFlowPollingError::Request)?
						.into()
					{
						MSADeviceFlowError::AuthorizationPending => {
							retry += 1;
							let remain = expires_in - (interval * retry);
							// use clipboard api in TAURI for this
							tracing::debug!(
								"(remain {}s) The user has not yet approved the authorization",
								remain
							);
							continue;
						}
						MSADeviceFlowError::AuthorizeDeclined => {
							break Err(MSADeviceFlowPollingError::AuthorizeDeclined)
						}
						MSADeviceFlowError::BadVerificationCode => {
							break Err(MSADeviceFlowPollingError::BadVerificationCode)
						}
						MSADeviceFlowError::ExpiredToken => {
							break Err(MSADeviceFlowPollingError::ExpiredToken(expires_in))
						}
						_ => unimplemented!(),
					}
				}

				let authed = req
					.map_future(|res| res.json::<MSADeviceAuthenticateReponse>())
					.await
					.map_err(MSADeviceFlowPollingError::Request)?;

				break Ok(authed);
			}
		});

	let msa_cert = match polling.await.expect("bugged") {
		Ok(authed) => {
			tracing::info!("MSA Verified.");
			authed
		}
		Err(reason) => {
			tracing::error!("(X) {}", reason);
			return Err(MSAuthError::PollingError(reason));
		}
	};

	let req = XboxLiveUserAuthenticateRequest::default().step(msa_cert);
	let xsts_token = Requestor::new(req).execute().await.unwrap();

	let xsts_token = match xsts_token
		.map_future(|xsts| xsts.json::<XboxLiveUserAuthenticateResponse>())
		.await
	{
		Ok(authed) => {
			tracing::info!("XSTS authorized.");
			authed
		}
		Err(reason) => {
			tracing::error!("(X) Failed authorizing XSTS!");
			tracing::error!("{:?}", reason);
			return Err(MSAuthError::XSTSError(reason));
		}
	};

	let req = XboxLiveSTSAuthorizeRequest::default().step(xsts_token);
	let mc_user_token = Requestor::new(req).execute().await.unwrap();

	let _mc_user_token = match mc_user_token
		.map_future(|token| token.json::<XboxLiveSTSAuthorizeResponse>())
		.await
	{
		Ok(authed) => {
			tracing::info!("Minecraft user authorized with XSTS.");
			authed
		}
		Err(reason) => {
			tracing::error!("(X) Failed authorizing Minecraft user with XSTS!");
			tracing::error!("{:?}", reason);
			return Err(MSAuthError::XSTSError(reason));
		}
	};

	Ok(_mc_user_token)
}

#[derive(Debug, thiserror::Error)]
pub enum MSADeviceFlowPollingError {
	#[error("User denied authorization.")]
	AuthorizeDeclined,
	#[error("`device_code` is wrong. This may be a bug.")]
	BadVerificationCode,
	#[error("`device_code` disabled due to exceeding `expires_in` seconds (It was set to {0} seconds.). Redo.")]
	ExpiredToken(i32),
	#[error("Request error occured {0:#?}")]
	Request(#[from] reqwest::Error),
	#[error("Unknown Error occured.")]
	Unknown,
}
