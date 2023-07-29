mod methods;
mod traits;

use traits::SubdomainScanner;

pub fn get_scanners() -> Vec<Box<dyn SubdomainScanner>> {
    vec![
        Box::new(methods::BruteForceScan::new()),
        Box::new(methods::CrtShScan::new()),
        Box::new(methods::ThreatCrowdScan::new()),
        Box::new(methods::ThreatMinerScan::new()),
        Box::new(methods::WebArchiveScan::new()),
    ]
}

#[allow(dead_code)]
pub fn bruteforce_scan() -> Box<dyn SubdomainScanner> {
    Box::new(methods::BruteForceScan::new())
}

#[allow(dead_code)]
pub fn crtsh_scan() -> Box<dyn SubdomainScanner> {
    Box::new(methods::CrtShScan::new())
}

#[allow(dead_code)]
pub fn threatcrowd_scan() -> Box<dyn SubdomainScanner> {
    Box::new(methods::ThreatCrowdScan::new())
}

#[allow(dead_code)]
pub fn threatminer_scan() -> Box<dyn SubdomainScanner> {
    Box::new(methods::ThreatMinerScan::new())
}

#[allow(dead_code)]
pub fn webarchive_scan() -> Box<dyn SubdomainScanner> {
    Box::new(methods::WebArchiveScan::new())
}
