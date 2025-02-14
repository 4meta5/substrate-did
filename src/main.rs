//! Substrate Node Template CLI library.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod chain_spec;
mod service;
mod cli;

pub use substrate_cli::{VersionInfo, IntoExit, error};

fn main() {
	let version = VersionInfo {
		name: "Identity Node",
		commit: env!("VERGEN_SHA_SHORT"),
		version: env!("CARGO_PKG_VERSION"),
		executable_name: "identity-chain",
		author: "Anonymous",
		description: "Experimental DID Node",
		support_url: "support.anonymous.an",
	};

	if let Err(e) = cli::run(::std::env::args(), cli::Exit, version) {
		eprintln!("Error starting the node: {}\n\n{:?}", e, e);
		std::process::exit(1)
	}
}
