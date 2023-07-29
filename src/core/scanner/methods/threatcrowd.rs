use crate::core::{
    error::ScanError,
    scanner::traits::{
        Scanner, SubdomainScanner,
    }
};

use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashSet;

pub struct ThreatCrowdScan {}

impl ThreatCrowdScan {
    pub fn new() -> Self {
        ThreatCrowdScan {}
    }
}

impl Scanner for ThreatCrowdScan {
    fn name(&self) -> String {
        String::from("Threatcrowd.org scanner")
    }
    fn about(&self) -> String {
        String::from("Finds subdomains using Threatcrowd.org's online database.")
    }
}

/// Json deserialization struct for retrieving results from response body
///
#[derive(Clone, Debug, Deserialize)]
struct ThreatCrowdResponse {
    subdomains: Vec<String>,
}

#[async_trait]
impl SubdomainScanner for ThreatCrowdScan {
    async fn get_subdomains(&self, target: &str) -> Result<Vec<String>, ScanError> {
        log::info!("Getting subdomains from threatcrowd.org...");

        let url = format!(
            "https://threatcrowd.org/searchApi/v2/domain/report/?domain={}",
            target
        );
        let res = reqwest::get(&url).await?;

        if !res.status().is_success() {
            return Err(ScanError::InvalidHttpResponse(self.name()));
        }

        let response: ThreatCrowdResponse = match res.json().await {
            Ok(info) => info,
            Err(_) => return Err(ScanError::InvalidHttpResponse(self.name())),
        };

        // We use a hashset to prevent duplication of data
        let subdomains: HashSet<String> = response
            .subdomains
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
