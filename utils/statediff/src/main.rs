// Copyright 2019-2022 ChainSafe Systems
// SPDX-License-Identifier: Apache-2.0, MIT
use directories::ProjectDirs;
use structopt::StructOpt;

use cid::Cid;
use forest_db::rocks::RocksDb;
use forest_db::rocks_config::RocksDbConfig;
use forest_statediff::print_state_diff;

/// Examine the state delta
#[derive(StructOpt)]
pub struct ChainCommand {
    /// The previous CID state root
    pre: Cid,
    /// The post CID state root
    post: Cid,
    /// The name of the chain
    #[structopt(short, long, default_value = "mainnet")]
    chain: String,
    /// The depth at which IPLD links are resolved
    #[structopt(short, long)]
    depth: Option<u64>,
}

impl ChainCommand {
    pub async fn run(&self) {
        let dir = ProjectDirs::from("com", "ChainSafe", "Forest").unwrap();
        let mut path = dir.data_dir().to_path_buf();
        path.push(&self.chain);
        path.push("db");

        let bs =
            RocksDb::open(path, &RocksDbConfig::default()).expect("Opening RocksDB must succeed");

        if let Err(err) = print_state_diff(&bs, &self.pre, &self.post, self.depth) {
            eprintln!("Failed to print state diff: {}", err);
        }
    }
}

/// statediff binary sub-commands available.
#[derive(StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::VersionlessSubcommands)]
enum Subcommand {
    #[structopt(name = "chain")]
    Chain(ChainCommand),
}

/// CLI structure generated when interacting with the statediff tool
#[derive(StructOpt)]
#[structopt(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    author = env!("CARGO_PKG_AUTHORS")
)]
struct Cli {
    #[structopt(subcommand)]
    cmd: Subcommand,
}

#[async_std::main]
async fn main() {
    // Capture Cli inputs
    let Cli { cmd } = Cli::from_args();
    match cmd {
        Subcommand::Chain(cmd) => {
            cmd.run().await;
        }
    }
}
