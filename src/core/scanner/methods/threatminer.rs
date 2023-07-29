use crate::core::{
    error::ScanError,
    scanner::traits::{
        Scanner, SubdomainScanner,
    }
};

use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashSet;

pub struct ThreatMinerScan {}

impl ThreatMinerScan {
    pub fn new() -> Self {
        ThreatMinerScan {}
    }
}

impl Scanner for ThreatMinerScan {
    fn name(&self) -> String {
        String::from("Threatminer scanner")
    }
    fn about(&self) -> String {
        String::from("Finds subdomains using threatminer.org's online api.")
    }
}

/// Json deserialization struct for retrieving results from response body
///
#[derive(Clone, Debug, Deserialize)]
struct ThreatMinerResponse {
    status_code: String,
    results: Vec<String>,
}

#[async_trait]
impl SubdomainScanner for ThreatMinerScan {
    async fn get_subdomains(&self, target: &str) -> Result<Vec<String>, ScanError> {
        log::info!("Getting subdomains from threatminer.org...");

        let url = format!(
            "https://api.threatminer.org/v2/domain.php?q={}&api=True&rt=5",
            target
        );
        let res = reqwest::get(&url).await?;

        if !res.status().is_success() {
            return Err(ScanError::InvalidHttpResponse(self.name()));
        }

        let response: ThreatMinerResponse = match res.json().await {
            Ok(info) => info,
            Err(_) => return Err(ScanError::InvalidHttpResponse(self.name())),
        };

        if response.status_code != "200" {
            return Err(ScanError::InvalidHttpResponse(format!(
                "{}. Status code: {}",
                self.name(),
                response.status_code
            )));
        }

        // we use a hashset to prevent duplication of data
        let subdomains: HashSet<String> = response
            .results
            .into_iter()
            .map(|entry| {
                entry
                    .split('\n')
                    .map(|subdomain| subdomain.trim().to_string())
                    .collect::<Vec<String>>()
            })
            .flatten()
            .filter(|subdomain: &String| !subdomain.contains('*'))
            .collect();

        Ok(subdomains.into_iter().collect())
    }
}
