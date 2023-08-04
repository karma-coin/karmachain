use futures::FutureExt;
use karmachain_node::{cli::Cli, service};
use sc_cli::CliConfiguration;
use serde::Deserialize;
use std::future::Future;
use tokio::{runtime::Handle, select};

pub const RPC_URL: &str = "http://localhost:9944/";

// TODO: fix this
// Cannot be called multiply times because of conflicting
// connections to RocksDB.
pub async fn create_runner<F, E>(cli: Cli, test: F) -> Result<(), E>
where
	F: Future<Output = Result<(), E>>,
{
	let command = &cli.run;
	let config = command.create_configuration(&cli, Handle::current()).unwrap();

	let mut task_manager = service::new_full(config, Default::default()).unwrap();

	let t1 = task_manager.future().fuse();
	let t2 = test;

	select! {
		_ = t1 => Ok(()),
		res = t2 => res,
	}
}

#[derive(Debug, Clone, Deserialize)]
pub struct RpcResponse<T> {
	pub id: u8,
	pub jsonrpc: String,
	pub result: Option<T>,
}
