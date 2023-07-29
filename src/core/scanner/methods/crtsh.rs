use crate::core::{
    error::ScanError,
    scanner::traits::{
        Scanner, SubdomainScanner,
    }
};

use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashSet;

pub struct CrtShScan {}

impl CrtShScan {
    pub fn new() -> Self {
        CrtShScan {}
    }
}

impl Scanner for CrtShScan {
    fn name(&self) -> String {
        String::from("Crt.sh scanner")
    }
    fn about(&self) -> String {
        String::from("Finds subdomains using crt.sh's online api.")
    }
}

/// Json deserialization struct for retrieving results from response body
///
#[derive(Clone, Debug, Deserialize)]
struct CrtShResponse {
    name_value: String,
}

#[async_trait]
impl SubdomainScanner for CrtShScan {
    async fn get_subdomains(&self, target: &str) -> Result<Vec<String>, ScanError> {
        log::info!("Getting subdomains from crt.sh...");

        let url = format!("https://crt.sh/?q=%25.{}&output=json", target);
        let res = reqwest::get(&url).await?;

        if !res.status().is_success() {
            return Err(ScanError::InvalidHttpResponse(self.name()));
        }

        let crtsh_entries: Vec<CrtShResponse> = match res.json().await {
            Ok(info) => info,
            Err(_) => return Err(ScanError::InvalidHttpResponse(self.name())),
        };

        // We use a hashset to prevent duplication of data
        let subdomains: HashSet<String> = crtsh_entries
            .into_iter()
            .map(|entry| {
                entry
                    .name_value
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
