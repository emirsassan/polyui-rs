use crate::{library::LibraryConfig, prisma::statistics};

use super::{utils::LibraryRequest, RouterBuilder};
use chrono::Utc;
use rspc::Type;
use serde::Deserialize;
use tokio::fs;
use uuid::Uuid;

pub(crate) fn mount() -> RouterBuilder {
	<RouterBuilder>::new()
		.query("list", |t| {
			t(|ctx, _: ()| async move { ctx.library_manager.get_all_libraries_config().await })
		})
		.library_query("getStatistics", |t| {
			t(|_, _: (), library| async move {
				let _statistics = library
					.db
					.statistics()
					.find_unique(statistics::id::equals(library.node_local_id))
					.exec()
					.await?;

				let library_db_size = match fs::metadata(library.config().data_directory()).await {
					Ok(metadata) => metadata.len(),
					Err(_) => 0,
				};

				use statistics::*;
				let params = vec![
					id::set(1),
					date_captured::set(Utc::now().into()),
					library_db_size::set(library_db_size.to_string()),
					total_bytes_used::set(0.to_string()),
				];

				Ok(library
					.db
					.statistics()
					.upsert(statistics::id::equals(1), params.clone(), params)
					.exec()
					.await?)
			})
		})
		.mutation("create", |t| {
			t(|ctx, name: String| async move {
				Ok(ctx
					.library_manager
					.create(LibraryConfig {
						name: name.to_string(),
						..Default::default()
					})
					.await?)
			})
		})
		.mutation("edit", |t| {
			#[derive(Type, Deserialize)]
			pub struct EditLibraryArgs {
				pub id: Uuid,
				pub name: Option<String>,
				pub description: Option<String>,
			}

			t(|ctx, args: EditLibraryArgs| async move {
				Ok(ctx
					.library_manager
					.edit(args.id, args.name, args.description)
					.await?)
			})
		})
		.mutation("delete", |t| {
			t(|ctx, id: Uuid| async move { Ok(ctx.library_manager.delete_library(id).await?) })
		})
}
