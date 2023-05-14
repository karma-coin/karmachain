use sc_cli::RunCmd;
use sp_rpc::ByPassToken;

#[derive(Debug, clap::Parser)]
pub struct Cli {
	#[command(subcommand)]
	pub subcommand: Option<Subcommand>,
	/// Default node arguments
	#[clap(flatten)]
	pub run: RunCmd,
	/// Default node arguments
	#[clap(flatten)]
	pub verifier_config: VerifierConfig,
}

#[derive(Debug, clap::Parser)]
pub struct VerifierConfig {
	/// Enable verifier mode.
	///
	/// The node will be provide verifier API
	#[arg(long)]
	pub verifier: bool,
	/// Bypass token
	///
	/// Token for verifier API to pass verification
	#[arg(long)]
	pub bypass_token: Option<ByPassToken>,
	/// Auth verifier endpoint url
	#[arg(long)]
	pub auth_dst: Option<String>,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommand {
	/// Key management cli utilities
	#[command(subcommand)]
	Key(sc_cli::KeySubcommand),

	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(sc_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),

	/// Sub-commands concerned with benchmarking.
	#[command(subcommand)]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// Try some command against runtime state.
	#[cfg(feature = "try-runtime")]
	TryRuntime(try_runtime_cli::TryRuntimeCmd),

	/// Try some command against runtime state. Note: `try-runtime` feature must be enabled.
	#[cfg(not(feature = "try-runtime"))]
	TryRuntime,

	/// Db meta columns information.
	ChainInfo(sc_cli::ChainInfoCmd),
}
