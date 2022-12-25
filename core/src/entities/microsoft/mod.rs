mod msa_device_authenticate;
mod msa_device_flow_error;
mod msa_device_verifing;

mod xbl_user_authenticate;
mod xbl_xsts_authorize;

pub trait FlowDependent {
	type Flowed;
	fn flow(&self) -> Self::Flowed;
}

pub trait FlowSteppable<T: FlowDependent> {
	fn step(self, dep: T) -> Self;
}

pub use self::{
	msa_device_authenticate::{MSADeviceAuthenticateReponse, MSADeviceAuthenticateRequest},
	msa_device_flow_error::{MSADeviceFlowError, MSADeviceFlowErrorResponse},
	msa_device_verifing::{MSADeviceVerifingRequest, MSADeviceVerifingResponse},
	xbl_user_authenticate::{
		XboxLiveUserAuthenticateProperty, XboxLiveUserAuthenticateRequest,
		XboxLiveUserAuthenticateResponse,
	},
	xbl_xsts_authorize::{
		XboxLiveSTSAuthorizeProperty, XboxLiveSTSAuthorizeRequest, XboxLiveSTSAuthorizeResponse,
	},
};
