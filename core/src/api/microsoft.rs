use super::RouterBuilder;

pub(crate) fn mount() -> RouterBuilder {
	<RouterBuilder>::new().query("get_msauth_token", |t| {
		t(|ctx, _: ()| async move { "token" })
	})
}
