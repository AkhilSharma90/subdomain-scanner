use async_trait::async_trait;

use crate::core::error::ScanError;

/// Scanner trait
pub trait Scanner {
    fn name(&self) -> String;
    fn about(&self) -> String;
}

/// SubdomainScanner trait. Required that the type implements the Scanner trait.
#[async_trait]
pub trait SubdomainScanner: Scanner {
    async fn get_subdomains(&self, target: &str) -> Result<Vec<String>, ScanError>;
}
