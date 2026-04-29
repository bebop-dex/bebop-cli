use clap::{Args, Parser, Subcommand, ValueEnum};

mod cache;
mod chains;
mod config;
mod quote;
mod tokens;
mod tui;
mod update;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, global = true)]
    output: Option<OutputFormat>,
    /// Verbose debug logging (prints outgoing request URLs and params)
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// RFQ quotes
    Quote {
        /// Request a firm, executable quote (mutually exclusive with --indicative)
        #[arg(long, group = "quote_type")]
        firm: bool,
        /// Request an indicative quote (default, mutually exclusive with --firm)
        #[arg(long, group = "quote_type")]
        indicative: bool,
        /// Token(s) to buy — symbol (e.g. USDC) or contract address (0x...). Repeat for multiple.
        #[arg(long, value_name = "SYMBOL | ADDRESS", num_args = 1..)]
        buy: Vec<String>,
        /// Token(s) to sell — symbol (e.g. WETH) or contract address (0x...). Repeat for multiple.
        #[arg(long, value_name = "SYMBOL | ADDRESS", num_args = 1..)]
        sell: Vec<String>,
        /// Amount(s) of the buy token(s) to receive (mutually exclusive with --amount-sell)
        #[arg(long, group = "amount", num_args = 1..)]
        amount_buy: Vec<String>,
        /// Amount(s) of the sell token(s) to spend (mutually exclusive with --amount-buy)
        #[arg(long, group = "amount", num_args = 1..)]
        amount_sell: Vec<String>,
        /// Chain name (e.g. ethereum, arbitrum); uses config value if not provided
        #[arg(short, long)]
        chain: Option<String>,
        /// Wallet address for the quote; uses config value if not provided
        #[arg(short, long)]
        wallet_address: Option<String>,
        /// API key for the quote; uses config value if not provided
        #[arg(long)]
        api_key: Option<String>,
        /// Source tag for the quote; uses config value if not provided
        #[arg(long)]
        source: Option<String>,
        /// Restrict makers — emoji(s) passed as include_makers (e.g. 🦊3, or 🦊2 🦊3)
        #[arg(long, value_name = "MAKER", num_args = 1..)]
        from: Vec<String>,
    },
    /// List tokens available for trading on Bebop PMM
    Tokens {
        /// Chain name (e.g. ethereum, arbitrum) or "all" for every chain; uses config value if not provided
        #[arg(short, long)]
        chain: Option<String>,
        /// Filter by symbol, name, or address (case-insensitive substring match)
        #[arg(short, long)]
        search: Option<String>,
    },
    /// Look up a single token by symbol or address
    Token {
        /// Symbol (e.g. USDC) or contract address (0x...)
        #[arg(value_name = "SYMBOL | ADDRESS")]
        query: String,
        /// Chain name (e.g. ethereum, arbitrum) or "all" for every chain; uses config value if not provided
        #[arg(short, long)]
        chain: Option<String>,
    },
    /// Returns a mapping of chain names to chain IDs supported by the RFQ API
    Chains {},
    /// Manage CLI configuration
    Config(ConfigArgs),
    /// MCP tool schema for AI agent discovery
    Manifest {},
    /// Update bebop to the latest version
    Update {},
}

#[derive(Args)]
#[command(args_conflicts_with_subcommands = true)]
struct ConfigArgs {
    /// Config key to display (api-key, source, wallet-address, chain, format)
    key: Option<String>,
    #[command(subcommand)]
    action: Option<ConfigAction>,
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Set a config value
    Set {
        /// Config key (api-key, source, wallet-address, chain, format)
        key: String,
        /// Value to set
        value: String,
    },
    /// Reset all config values
    Reset,
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum OutputFormat {
    Json,
    Text,
}

fn require_chain(chain: Option<String>, cfg: &config::Config) -> String {
    chain.or_else(|| cfg.chain.clone()).unwrap_or_else(|| {
        eprintln!("error: --chain is required (or set via `config set chain <name>`)");
        std::process::exit(1);
    })
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let cfg = config::Config::load();
    let verbose = cli.verbose;

    let output = cli.output.unwrap_or_else(|| {
        match cfg.format.as_deref() {
            Some("text") => OutputFormat::Text,
            _ => OutputFormat::Json,
        }
    });

    match cli.command {
        Some(Commands::Config(args)) => {
            match args.action {
                Some(ConfigAction::Set { key, value }) => config::set(&key, &value),
                Some(ConfigAction::Reset) => config::reset(),
                None => config::show(args.key.as_deref(), &output),
            }
        }
        Some(Commands::Chains {}) => {
            chains::list(&output).await;
        }
        Some(Commands::Tokens { chain, search }) => {
            let chain = require_chain(chain, &cfg);
            tokens::list(&chain, search.as_deref(), &output).await;
        }
        Some(Commands::Quote { firm, indicative: _, buy, sell, amount_buy, amount_sell, chain, wallet_address, api_key, source, from }) => {
            let chain = require_chain(chain, &cfg);
            let wallet_address = wallet_address.or_else(|| cfg.wallet_address.clone())
                .unwrap_or_else(|| "0x0000000000000000000000000000000000000000".to_string());
            let api_key = api_key.or_else(|| cfg.api_key.clone());
            let source = source.or_else(|| cfg.source.clone());
            if firm {
                quote::firm::quote(&buy, &sell, &amount_buy, &amount_sell, &chain, &wallet_address, api_key.as_deref(), source.as_deref(), &from, verbose, &output).await;
            } else {
                println!("indicative quotes not yet implemented");
            }
        }
        Some(Commands::Token { query, chain }) => {
            let chain = require_chain(chain, &cfg);
            tokens::get(&query, &chain, &output).await;
        }
        Some(Commands::Update {}) => {
            update::update();
        }
        Some(Commands::Manifest {}) => {}
        None => {
            use std::io::IsTerminal;
            if std::io::stdout().is_terminal() {
                if let Err(e) = tui::run().await {
                    eprintln!("TUI error: {e}");
                    std::process::exit(1);
                }
            } else {
                use clap::CommandFactory;
                Cli::command().print_help().ok();
                println!();
            }
        }
    }
}
