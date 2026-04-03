use std::collections::BTreeMap;
use tabled::{Table, Tabled, settings::Style};

use crate::OutputFormat;

#[derive(Tabled)]
struct ChainRow {
    name: String,
    #[tabled(rename = "chain_id")]
    id: u64,
}

pub async fn fetch() -> BTreeMap<String, u64> {
    if let Some(cached) = crate::cache::read("chains") {
        return serde_json::from_str(&cached).expect("failed to parse cached chains");
    }
    let data = reqwest::get("https://api.bebop.xyz/pmm/chains")
        .await
        .expect("failed to fetch chains")
        .json::<BTreeMap<String, u64>>()
        .await
        .expect("failed to parse chains response");
    crate::cache::write("chains", &serde_json::to_string(&data).unwrap());
    data
}

pub async fn try_fetch() -> Result<BTreeMap<String, u64>, String> {
    if let Some(cached) = crate::cache::read("chains") {
        return serde_json::from_str(&cached).map_err(|e| e.to_string());
    }
    let data = reqwest::get("https://api.bebop.xyz/pmm/chains")
        .await
        .map_err(|e| e.to_string())?
        .json::<BTreeMap<String, u64>>()
        .await
        .map_err(|e| e.to_string())?;
    crate::cache::write("chains", &serde_json::to_string(&data).unwrap());
    Ok(data)
}

pub async fn list(output: &OutputFormat) {
    let spinner = indicatif::ProgressBar::new_spinner();
    spinner.set_message("Fetching chains...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    let chains = fetch().await;

    spinner.finish_and_clear();
    match output {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&chains).unwrap());
        }
        OutputFormat::Text => {
            let rows: Vec<ChainRow> = chains.into_iter()
                .map(|(name, id)| ChainRow { name, id })
                .collect();
            let mut table = Table::new(rows);
            table.with(Style::rounded());
            println!("{}", table);
        }
    }
}
