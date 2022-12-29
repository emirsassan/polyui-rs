use super::{RouterBuilder, Ctx};
use tokio::sync::oneshot::Sender;

pub(crate) fn mount() -> RouterBuilder {
	<RouterBuilder>::new()
		.mutation("authenticate", |t| {
			t(|ctx: Ctx, browser_url: Sender<url::Url>| async move {
				Ok(crate::helpers::msapi::authenticate(browser_url).await?)
			})
		})
}
