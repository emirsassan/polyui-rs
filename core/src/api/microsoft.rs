use super::{RouterBuilder, Ctx};

pub(crate) fn mount() -> RouterBuilder {
	<RouterBuilder>::new()
	/*
		.mutation("authenticate", |t| {
			t(|ctx: Ctx, browser_url: &str| async move {
				Ok(crate::helpers::msapi::authenticate(url::Url::parse(browser_url)).await?)
			})
		})
		*/
}
