use serde::{Deserialize, Serialize};
use tabled::{Table, Tabled, settings::Style};

use crate::OutputFormat;

#[derive(Deserialize)]
struct TokenList {
    tokens: Vec<Token>,
}

#[derive(Deserialize, Serialize, Tabled, Clone)]
pub struct Token {
    pub symbol: String,
    pub name: String,
    pub address: String,
    pub decimals: u8,
}

#[derive(Serialize, Tabled)]
struct TokenWithChain {
    chain: String,
    symbol: String,
    name: String,
    address: String,
    decimals: u8,
}

async fn fetch_all_tokens() -> Vec<TokenWithChain> {
    let chains = crate::chains::fetch().await;
    let mut all = Vec::new();
    for chain_name in chains.keys() {
        let tokens = fetch_tokens(chain_name).await;
        for t in tokens {
            all.push(TokenWithChain {
                chain: chain_name.clone(),
                symbol: t.symbol,
                name: t.name,
                address: t.address,
                decimals: t.decimals,
            });
        }
    }
    all.sort_by(|a, b| a.symbol.to_lowercase().cmp(&b.symbol.to_lowercase()));
    all
}

async fn unknown_chain(chain: &str) -> ! {
    eprintln!("error: unknown chain '{}'", chain);
    let chains = crate::chains::fetch().await;
    let names: Vec<&str> = chains.keys().map(|s| s.as_str()).collect();
    let input = chain.to_lowercase();
    let suggestions: Vec<&&str> = names.iter()
        .filter(|n| n.starts_with(&input) || input.starts_with(*n) || n.contains(&input) || input.contains(*n))
        .collect();
    if !suggestions.is_empty() {
        eprintln!("did you mean: {}?", suggestions.iter().map(|s| **s).collect::<Vec<_>>().join(", "));
    }
    eprintln!("available chains: {}", names.join(", "));
    std::process::exit(1);
}

pub async fn fetch_tokens(chain: &str) -> Vec<Token> {
    let cache_key = format!("tokens_{chain}");
    let mut tokens = if let Some(cached) = crate::cache::read(&cache_key) {
        serde_json::from_str(&cached).expect("failed to parse cached tokens")
    } else {
        let url = format!("https://api.bebop.xyz/pmm/{chain}/v3/tokenlist");
        let resp = reqwest::get(&url).await.expect("failed to fetch tokenlist");
        if !resp.status().is_success() {
            unknown_chain(chain).await;
        }
        let tokens = resp.json::<TokenList>().await.expect("failed to parse tokenlist response").tokens;
        crate::cache::write(&cache_key, &serde_json::to_string(&tokens).unwrap());
        tokens
    };
    tokens.sort_by(|a, b| a.symbol.to_lowercase().cmp(&b.symbol.to_lowercase()));
    tokens
}

pub async fn try_fetch_tokens(chain: &str) -> Result<Vec<Token>, String> {
    let cache_key = format!("tokens_{chain}");
    let mut tokens = if let Some(cached) = crate::cache::read(&cache_key) {
        serde_json::from_str(&cached).map_err(|e| e.to_string())?
    } else {
        let url = format!("https://api.bebop.xyz/pmm/{chain}/v3/tokenlist");
        let resp = reqwest::get(&url).await.map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("unknown chain '{chain}' (HTTP {})", resp.status()));
        }
        let tokens = resp
            .json::<TokenList>()
            .await
            .map_err(|e| e.to_string())?
            .tokens;
        crate::cache::write(&cache_key, &serde_json::to_string(&tokens).unwrap());
        tokens
    };
    tokens.sort_by(|a, b| a.symbol.to_lowercase().cmp(&b.symbol.to_lowercase()));
    Ok(tokens)
}

pub fn resolve<'a>(query: &str, tokens: &'a [Token]) -> &'a Token {
    let q = query.to_lowercase();
    let is_address = q.starts_with("0x");

    let matches: Vec<&Token> = tokens.iter().filter(|t| {
        if is_address {
            t.address.to_lowercase() == q
        } else {
            t.symbol.to_lowercase() == q
        }
    }).collect();

    match matches.len() {
        0 => {
            eprintln!("error: token '{}' not found", query);
            std::process::exit(1);
        }
        1 => matches[0],
        n => {
            eprintln!("error: {} tokens match '{}'; use an address instead:", n, query);
            for t in &matches {
                eprintln!("  {} — {}", t.address, t.symbol);
            }
            std::process::exit(1);
        }
    }
}

pub async fn get(query: &str, chain: &str, output: &OutputFormat) {
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message("Fetching tokens...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    if chain == "all" {
        let all = fetch_all_tokens().await;
        spinner.finish_and_clear();
        let q = query.to_lowercase();
        let is_address = q.starts_with("0x");
        let matches: Vec<&TokenWithChain> = all.iter().filter(|t| {
            if is_address {
                t.address.to_lowercase() == q
            } else {
                t.symbol.to_lowercase() == q
            }
        }).collect();

        if matches.is_empty() {
            eprintln!("error: token '{}' not found on any chain", query);
            std::process::exit(1);
        }

        match output {
            OutputFormat::Json => {
                let json: Vec<serde_json::Value> = matches.iter().map(|t| {
                    serde_json::json!({
                        "chain": t.chain,
                        "symbol": t.symbol,
                        "name": t.name,
                        "address": t.address,
                        "decimals": t.decimals,
                    })
                }).collect();
                println!("{}", serde_json::to_string_pretty(&json).unwrap());
            }
            OutputFormat::Text => {
                let mut table = Table::new(&matches);
                table.with(Style::rounded());
                println!("{}", table);
            }
        }
        return;
    }

    let tokens = fetch_tokens(chain).await;
    spinner.finish_and_clear();
    let t = resolve(query, &tokens);
    match output {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&serde_json::json!({
                "symbol": t.symbol,
                "name": t.name,
                "address": t.address,
                "decimals": t.decimals,
            })).unwrap());
        }
        OutputFormat::Text => {
            let matches = vec![t];
            let mut table = Table::new(&matches);
            table.with(Style::rounded());
            println!("{}", table);
        }
    }
}

pub async fn list(chain: &str, search: Option<&str>, output: &OutputFormat) {
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message("Fetching tokens...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    if chain == "all" {
        let all = fetch_all_tokens().await;
        spinner.finish_and_clear();

        let tokens: Vec<&TokenWithChain> = all.iter().filter(|t| {
            if let Some(q) = search {
                let q = q.to_lowercase();
                t.symbol.to_lowercase().contains(&q)
                    || t.name.to_lowercase().contains(&q)
                    || t.address.to_lowercase().contains(&q)
            } else {
                true
            }
        }).collect();

        match output {
            OutputFormat::Json => {
                let json: Vec<serde_json::Value> = tokens.iter().map(|t| {
                    serde_json::json!({
                        "chain": t.chain,
                        "symbol": t.symbol,
                        "name": t.name,
                        "address": t.address,
                        "decimals": t.decimals,
                    })
                }).collect();
                println!("{}", serde_json::to_string_pretty(&json).unwrap());
            }
            OutputFormat::Text => {
                let mut table = Table::new(tokens);
                table.with(Style::rounded());
                println!("{}", table);
            }
        }
        return;
    }

    let all = fetch_tokens(chain).await;
    spinner.finish_and_clear();

    let tokens: Vec<&Token> = all.iter().filter(|t| {
        if let Some(q) = search {
            let q = q.to_lowercase();
            t.symbol.to_lowercase().contains(&q)
                || t.name.to_lowercase().contains(&q)
                || t.address.to_lowercase().contains(&q)
        } else {
            true
        }
    }).collect();

    match output {
        OutputFormat::Json => {
            let json: Vec<serde_json::Value> = tokens.iter().map(|t| {
                serde_json::json!({
                    "symbol": t.symbol,
                    "name": t.name,
                    "address": t.address,
                    "decimals": t.decimals,
                })
            }).collect();
            println!("{}", serde_json::to_string_pretty(&json).unwrap());
        }
        OutputFormat::Text => {
            let mut table = Table::new(tokens);
            table.with(Style::rounded());
            println!("{}", table);
        }
    }
}
