use super::models::Subdomain;
use std::sync::Arc;
use std::time::Duration;

use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    name_server::{GenericConnection, GenericConnectionProvider, TokioRuntime},
    AsyncResolver,
};

pub type Resolver = Arc<AsyncResolver<GenericConnection, GenericConnectionProvider<TokioRuntime>>>;

/// Creates a new trust_dns_resolver
pub fn new_dns_resolver() -> Resolver {
    log::info!("Generating dns resolver...");

    let resolver = AsyncResolver::tokio(
        ResolverConfig::default(),
        ResolverOpts {
            timeout: Duration::from_secs(4),
            ..Default::default()
        },
    )
    .expect("Building dns resolver failed!");
    
    Arc::new(resolver)
}

/// Check to see if a subdomain resolves according to the Domain Naming System
pub async fn resolve_dns(dns_resolver: &Resolver, subdomain: Subdomain) -> Option<Subdomain> {
    match &dns_resolver
        .lookup_ip(subdomain.domain_name.as_str())
        .await
        .is_ok()
    {
        true => Some(subdomain),
        false => None,
    }
}
