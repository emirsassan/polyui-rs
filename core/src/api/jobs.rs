use super::{utils::LibraryRequest, RouterBuilder};
use crate::job::JobManager;

pub(crate) fn mount() -> RouterBuilder {
	<RouterBuilder>::new()
		.library_query("getRunning", |t| {
			t(|ctx, _: (), _| async move { Ok(ctx.jobs.get_running().await) })
		})
		.library_query("isRunning", |t| {
			t(|ctx, _: (), _| async move { Ok(!ctx.jobs.get_running().await.is_empty()) })
		})
		.library_query("getHistory", |t| {
			t(|_, _: (), library| async move { Ok(JobManager::get_history(&library).await?) })
		})
}
