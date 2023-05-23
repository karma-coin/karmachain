use futures::FutureExt;
use karmachain_node::{
	cli::{Cli, VerifierConfig},
	service,
};
use sc_cli::{CliConfiguration, SubstrateCli};
use std::future::Future;
use tokio::{runtime::Handle, select};

// TODO: fix this
// Cannot be called multiply times because of conflicting
// connections to RocksDB.
pub async fn create_runner<F, E>(test: F) -> Result<(), E>
where
	F: Future<Output = Result<(), E>>,
{
	let mut cli = Cli::from_args();
	cli.run.shared_params.dev = true;
	let command = &cli.run;
	let config = command.create_configuration(&cli, Handle::current()).unwrap();
	let verifier_config = VerifierConfig { verifier: false, bypass_token: None, auth_dst: None };

	let mut task_manager = service::new_full(config, verifier_config).unwrap();

	let t1 = task_manager.future().fuse();
	let t2 = test;

	select! {
		_ = t1 => Ok(()),
		res = t2 => res,
	}
}
