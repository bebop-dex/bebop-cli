use std::time::{SystemTime, UNIX_EPOCH};

use futures::future::join_all;
use serde_json::json;
use tabled::{Table, Tabled, settings::{Style, width::Width, peaker::Priority, Remove, object::Columns}};

use crate::OutputFormat;
use crate::tokens::Token;
use super::types::{QuoteApiResponse, ErrorApiResponse};

fn to_base_units(human_amount: &str, decimals: u8) -> String {
    let parts: Vec<&str> = human_amount.split('.').collect();
    let integer = parts[0];
    let fraction = if parts.len() > 1 { parts[1] } else { "" };

    let d = decimals as usize;
    let padded = if fraction.len() >= d {
        format!("{}{}", integer, &fraction[..d])
    } else {
        format!("{}{}{}", integer, fraction, "0".repeat(d - fraction.len()))
    };

    let trimmed = padded.trim_start_matches('0');
    if trimmed.is_empty() { "0".to_string() } else { trimmed.to_string() }
}

fn from_base_units(base_amount: &str, decimals: u8) -> String {
    let d = decimals as usize;
    let padded = if base_amount.len() <= d {
        format!("{}{}", "0".repeat(d - base_amount.len() + 1), base_amount)
    } else {
        base_amount.to_string()
    };
    let (integer, fraction) = padded.split_at(padded.len() - d);
    let fraction = fraction.trim_end_matches('0');
    if fraction.is_empty() {
        integer.to_string()
    } else {
        format!("{}.{}", integer, fraction)
    }
}

#[derive(Tabled)]
struct QuoteSummary {
    #[tabled(rename = "Sell")]
    sell: String,
    #[tabled(rename = "Buy")]
    buy: String,
    #[tabled(rename = "Rate")]
    rate: String,
    #[tabled(rename = "Impact")]
    impact: String,
    #[tabled(rename = "Makers")]
    makers: String,
    #[tabled(rename = "Expires in")]
    ttl: String,
    #[tabled(rename = "Error")]
    error: String,
}

struct QuoteOutcome {
    buy_label: String,
    sell_label: String,
    result: Result<QuoteApiResponse, String>,
}

async fn fetch_quote(
    buy_token: &Token,
    sell_token: &Token,
    amount_buy: Option<&str>,
    amount_sell: Option<&str>,
    chain: &str,
    wallet_address: &str,
    api_key: Option<&str>,
) -> Result<QuoteApiResponse, String> {
    let mut params = vec![
        ("buy_tokens".to_string(), buy_token.address.clone()),
        ("sell_tokens".to_string(), sell_token.address.clone()),
        ("taker_address".to_string(), wallet_address.to_string()),
        ("gasless".to_string(), "false".to_string()),
    ];

    if let Some(amt) = amount_sell {
        params.push(("sell_amounts".to_string(), to_base_units(amt, sell_token.decimals)));
    } else if let Some(amt) = amount_buy {
        params.push(("buy_amounts".to_string(), to_base_units(amt, buy_token.decimals)));
    }

    let url = format!("https://api.bebop.xyz/pmm/{chain}/v3/quote");
    let client = reqwest::Client::new();
    let mut req = client.get(&url).query(&params);
    if let Some(key) = api_key {
        req = req.header("Authorization", key);
    }
    let resp = req.send().await.expect("failed to request quote");

    if !resp.status().is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("quote request failed: {}", body));
    }

    let raw: serde_json::Value = resp.json().await.expect("failed to parse quote response");

    if raw.get("error").is_some() {
        let err: ErrorApiResponse = serde_json::from_value(raw).expect("failed to parse error");
        return Err(format!("{} (code {})", err.error.message.unwrap_or_default(), err.error.error_code));
    }

    let quote: QuoteApiResponse = serde_json::from_value(raw).expect("failed to deserialize quote");

    if quote.status != "SIG_SUCCESS" {
        let mut msg = format!("quote failed with status: {}", quote.status);
        if let Some(info) = &quote.info {
            msg.push_str(&format!(": {}", info));
        }
        return Err(msg);
    }

    Ok(quote)
}

pub async fn quote(
    buys: &[String],
    sells: &[String],
    amounts_buy: &[String],
    amounts_sell: &[String],
    chain: &str,
    wallet_address: &str,
    api_key: Option<&str>,
    output: &OutputFormat,
) {
    let authenticated = api_key.is_some();
    if !authenticated {
        eprintln!("\x1b[33mwarning: {}\x1b[0m", UNAUTHENTICATED_WARNING);
    }

    if buys.is_empty() || sells.is_empty() {
        eprintln!("error: at least one --buy and one --sell token required");
        std::process::exit(1);
    }
    if buys.len() > 1 && sells.len() > 1 {
        eprintln!("error: cannot have multiple --buy and multiple --sell at the same time");
        std::process::exit(1);
    }
    if amounts_buy.is_empty() && amounts_sell.is_empty() {
        eprintln!("error: provide --amount-buy or --amount-sell");
        std::process::exit(1);
    }

    let tokens = crate::tokens::fetch_tokens(chain).await;

    let buy_tokens: Vec<&Token> = buys.iter().map(|b| crate::tokens::resolve(b, &tokens)).collect();
    let sell_tokens: Vec<&Token> = sells.iter().map(|s| crate::tokens::resolve(s, &tokens)).collect();

    // Build futures for all quote requests, then execute in parallel.
    let mut futs: Vec<std::pin::Pin<Box<dyn std::future::Future<Output = Result<QuoteApiResponse, String>> + '_>>> = Vec::new();
    let mut labels: Vec<(&str, &str)> = Vec::new();

    if buys.len() > 1 {
        for (i, bt) in buy_tokens.iter().enumerate() {
            let ab = amounts_buy.get(i).map(|s| s.as_str());
            let as_ = amounts_sell.first().map(|s| s.as_str());
            futs.push(Box::pin(fetch_quote(bt, sell_tokens[0], ab, as_, chain, wallet_address, api_key)));
            labels.push((&buys[i], &sells[0]));
        }
    } else if sells.len() > 1 {
        for (i, st) in sell_tokens.iter().enumerate() {
            let ab = amounts_buy.first().map(|s| s.as_str());
            let as_ = amounts_sell.get(i).map(|s| s.as_str());
            futs.push(Box::pin(fetch_quote(buy_tokens[0], st, ab, as_, chain, wallet_address, api_key)));
            labels.push((&buys[0], &sells[i]));
        }
    } else {
        let ab = amounts_buy.first().map(|s| s.as_str());
        let as_ = amounts_sell.first().map(|s| s.as_str());
        futs.push(Box::pin(fetch_quote(buy_tokens[0], sell_tokens[0], ab, as_, chain, wallet_address, api_key)));
        labels.push((&buys[0], &sells[0]));
    }

    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message("Fetching quotes...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    let results = join_all(futs).await;

    spinner.finish_and_clear();

    let outcomes: Vec<QuoteOutcome> = results
        .into_iter()
        .zip(labels)
        .map(|(result, (buy_label, sell_label))| QuoteOutcome {
            buy_label: buy_label.to_string(),
            sell_label: sell_label.to_string(),
            result,
        })
        .collect();

    if outcomes.iter().all(|o| o.result.is_err()) {
        eprintln!("error: all quotes failed");
        std::process::exit(1);
    }

    match output {
        OutputFormat::Json => {
            print_quotes_json(&outcomes, authenticated);
        }
        OutputFormat::Text => {
            print_quotes_text(&outcomes);
        }
    }
}

fn ttl_seconds(expiry: u64) -> i64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    expiry as i64 - now as i64
}

const UNAUTHENTICATED_WARNING: &str = "API key not configured. Unauthenticated requests receive significantly worse prices and are heavily rate limited.";

fn quote_to_json(quote: &QuoteApiResponse) -> serde_json::Value {
    let sell = quote.sell_tokens.values().next().expect("no sell token in response");
    let buy = quote.buy_tokens.values().next().expect("no buy token in response");

    let sell_human = from_base_units(&sell.amount, sell.decimals);
    let buy_human = from_base_units(&buy.amount, buy.decimals);
    let buy_min_human = from_base_units(&buy.minimum_amount, buy.decimals);

    let sell_f = sell_human.parse::<f64>().unwrap_or(0.0);
    let buy_f = buy_human.parse::<f64>().unwrap_or(0.0);

    let rate = if sell_f > 0.0 { Some(buy_f / sell_f) } else { None };

    let mut out = json!({
        "sell": { "symbol": sell.symbol, "amount": sell_human },
        "buy": { "symbol": buy.symbol, "amount": buy_human, "minimum_amount": buy_min_human },
        "makers": quote.makers,
        "ttl_seconds": ttl_seconds(quote.expiry),
        "quote_id": quote.quote_id,
    });

    if let (Some(sell_usd), Some(buy_usd)) = (sell.price_usd, buy.price_usd) {
        out["sell"]["usd"] = json!(sell_usd * sell_f);
        out["buy"]["usd"] = json!(buy_usd * buy_f);
    }
    if let Some(r) = rate {
        out["rate"] = json!(r);
    }
    if let Some(impact) = quote.price_impact {
        out["price_impact_pct"] = json!(impact * 100.0);
    }

    out
}

fn print_quotes_json(outcomes: &[QuoteOutcome], authenticated: bool) {
    let items: Vec<serde_json::Value> = outcomes.iter().map(|o| {
        match &o.result {
            Ok(quote) => quote_to_json(quote),
            Err(e) => json!({
                "sell": { "symbol": o.sell_label },
                "buy": { "symbol": o.buy_label },
                "error": e,
            }),
        }
    }).collect();

    let mut out = json!({ "authenticated": authenticated, "quotes": items });
    if !authenticated {
        out["warning"] = json!(UNAUTHENTICATED_WARNING);
    }

    println!("{}", serde_json::to_string_pretty(&out).unwrap());
}

/// Position (1-indexed) of the first non-zero digit after the decimal point.
/// Returns 0 when the number has no fractional part or all decimals are zero.
fn first_nonzero_decimal_pos(s: &str) -> usize {
    match s.find('.') {
        Some(dot) => s[dot + 1..]
            .find(|c: char| c != '0')
            .map(|p| p + 1)
            .unwrap_or(0),
        None => 0,
    }
}

fn integer_part_len(s: &str) -> usize {
    s.find('.').unwrap_or(s.len())
}

/// Format a decimal string so that the integer part is right-padded to
/// `int_width` and the fractional part is exactly `decimals` digits.
fn align_decimal(s: &str, int_width: usize, decimals: usize) -> String {
    let (int, frac) = match s.find('.') {
        Some(dot) => (&s[..dot], &s[dot + 1..]),
        None => (s, ""),
    };
    let padded_frac = if frac.len() >= decimals {
        frac[..decimals].to_string()
    } else {
        format!("{}{}", frac, "0".repeat(decimals - frac.len()))
    };
    format!("{:>w$}.{}", int, padded_frac, w = int_width)
}

fn print_quotes_text(outcomes: &[QuoteOutcome]) {
    // First pass: extract numeric data from successful quotes.
    struct Row {
        sell_human: String,
        sell_symbol: String,
        sell_usd: Option<f64>,
        sell_f: f64,
        buy_human: String,
        buy_symbol: String,
        buy_usd: Option<f64>,
        buy_f: f64,
        rate: Option<f64>,
        price_impact: Option<f64>,
        makers: String,
        ttl: i64,
    }

    let row_data: Vec<Option<Row>> = outcomes.iter().map(|o| {
        let q = match &o.result {
            Ok(q) => q,
            Err(_) => return None,
        };
        let sell = q.sell_tokens.values().next().expect("no sell token");
        let buy = q.buy_tokens.values().next().expect("no buy token");
        let sell_human = from_base_units(&sell.amount, sell.decimals);
        let buy_human = from_base_units(&buy.amount, buy.decimals);
        let sell_f = sell_human.parse::<f64>().unwrap_or(0.0);
        let buy_f = buy_human.parse::<f64>().unwrap_or(0.0);
        Some(Row {
            sell_human,
            sell_symbol: sell.symbol.clone(),
            sell_usd: sell.price_usd,
            sell_f,
            buy_human,
            buy_symbol: buy.symbol.clone(),
            buy_usd: buy.price_usd,
            buy_f,
            rate: if sell_f > 0.0 { Some(buy_f / sell_f) } else { None },
            price_impact: q.price_impact,
            makers: q.makers.join(" "),
            ttl: ttl_seconds(q.expiry),
        })
    }).collect();

    let successful_rows: Vec<&Row> = row_data.iter().filter_map(|r| r.as_ref()).collect();
    let has_errors = outcomes.iter().any(|o| o.result.is_err());

    // Compute buy-amount decimal precision: max first-nonzero position, at least 5.
    let buy_prec = successful_rows.iter()
        .map(|r| first_nonzero_decimal_pos(&r.buy_human))
        .max().unwrap_or(0).max(5);

    let buy_int_w = successful_rows.iter()
        .map(|r| integer_part_len(&format!("{:.prec$}", r.buy_f, prec = buy_prec)))
        .max().unwrap_or(1);

    // Compute rate decimal precision the same way.
    let rate_prec = successful_rows.iter()
        .filter_map(|r| r.rate.map(|v| first_nonzero_decimal_pos(&format!("{:.20}", v))))
        .max().unwrap_or(0).max(5);

    let rate_int_w = successful_rows.iter()
        .filter_map(|r| r.rate.map(|v| integer_part_len(&format!("{:.prec$}", v, prec = rate_prec))))
        .max().unwrap_or(1);

    // Second pass: build aligned display rows.
    let summary: Vec<QuoteSummary> = outcomes.iter().zip(row_data.iter()).map(|(o, row_opt)| {
        match row_opt {
            Some(r) => {
                let buy_fmt = format!("{:.prec$}", r.buy_f, prec = buy_prec);
                let buy_aligned = align_decimal(&buy_fmt, buy_int_w, buy_prec);
                let buy_str = match r.buy_usd {
                    Some(p) => format!("{} {} (${:.2})", buy_aligned, r.buy_symbol, p * r.buy_f),
                    None => format!("{} {}", buy_aligned, r.buy_symbol),
                };

                let rate_str = match r.rate {
                    Some(v) => {
                        let rate_fmt = format!("{:.prec$}", v, prec = rate_prec);
                        let rate_aligned = align_decimal(&rate_fmt, rate_int_w, rate_prec);
                        format!("1 {} = {} {}", r.sell_symbol, rate_aligned, r.buy_symbol)
                    }
                    None => "-".to_string(),
                };

                QuoteSummary {
                    sell: match r.sell_usd {
                        Some(p) => format!("{} {} (${:.2})", r.sell_human, r.sell_symbol, p * r.sell_f),
                        None => format!("{} {}", r.sell_human, r.sell_symbol),
                    },
                    buy: buy_str,
                    rate: rate_str,
                    impact: match r.price_impact {
                        Some(i) => format!("{:.4}%", i * 100.0),
                        None => "-".to_string(),
                    },
                    makers: r.makers.clone(),
                    ttl: format!("{}s", r.ttl),
                    error: String::new(),
                }
            }
            None => {
                let error_msg = match &o.result {
                    Err(e) => e.clone(),
                    _ => unreachable!(),
                };
                QuoteSummary {
                    sell: o.sell_label.clone(),
                    buy: o.buy_label.clone(),
                    rate: "-".to_string(),
                    impact: "-".to_string(),
                    makers: "-".to_string(),
                    ttl: "-".to_string(),
                    error: error_msg,
                }
            }
        }
    }).collect();

    let term_width = terminal_size::terminal_size()
        .map(|(w, _)| w.0 as usize)
        .unwrap_or(120);

    let mut table = Table::new(summary);
    table.with(Style::rounded());
    if !has_errors {
        table.with(Remove::column(Columns::last()));
    }
    table.with(Width::wrap(term_width).priority(Priority::max(false)).keep_words(true));
    println!("{}", table);

    for o in outcomes {
        if let Ok(q) = &o.result {
            for w in &q.warnings {
                eprintln!("\x1b[33mwarning: {}\x1b[0m", w.message);
            }
        }
    }
}
