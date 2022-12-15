use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use rspc::{Config, Type};
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use utils::{InvalidRequests, InvalidateOperationEvent};

pub type Router = rspc::Router<Ctx>;
pub(crate) type RouterBuilder = rspc::RouterBuilder<Ctx>;

#[derive(Debug, Clone, Serialize, Type)]
pub enum CoreEvent {
	NewThumbnail { cas_id: String },
	InvalidateOperation(InvalidateOperationEvent),
	InvalidateOperationDebounced(InvalidateOperationEvent),
}

pub struct Ctx {
    pub event_bus: broadcast::Sender<CoreEvent>
}

pub mod utils;

pub(crate) fn mount() -> Arc<Router> {
    let config = Config::new().set_ts_bindings_header("/* eslint-disable */");

    let config = config.export_ts_bindings(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../packages/client/src/core.ts"),
    );

    let r = <Router>::new()
        .config(config)
        .query("buildInfo", |t| {
            #[derive(Serialize, Type)]
            pub struct BuildInfo {
                version: &'static str,
                commit: &'static str,
            }

            t(|_, _: ()| BuildInfo {
                version: env!("CARGO_PKG_VERSION"),
                commit: env!("GIT_HASH"),
            })
        })
        .subscription("invalidateQuery", |t| {
            t(|ctx, _: ()| {
                let mut event_bus_rx = ctx.event_bus.subscribe();
                let mut last = Instant::now();
                async_stream::stream! {
                    while let Ok(event) = event_bus_rx.recv().await {
                        match event {
                            CoreEvent::InvalidateOperation(op) => yield op,
							CoreEvent::InvalidateOperationDebounced(op) => {
								let current = Instant::now();
								if current.duration_since(last) > Duration::from_millis(1000 / 60) {
									last = current;
									yield op;
								}
							},
							_ => {}
                        }
                    }
                }
            })
        })
        .build()
        .arced();
        InvalidRequests::validate(r.clone());

        r
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_and_export_rspc_bindings() {
        super::mount();
    }
}