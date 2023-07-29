use crate::core::{
    error::ScanError,
    scanner::traits::{
        Scanner, SubdomainScanner,
    }
};

use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Instant,
};

use async_trait::async_trait;
use futures::StreamExt;
use tokio::sync::mpsc;

pub struct BruteForceScan {}

impl BruteForceScan {
    pub fn new() -> Self {
        BruteForceScan {}
    }
}

impl Scanner for BruteForceScan {
    fn name(&self) -> String {
        String::from("Brute force scanner")
    }
    fn about(&self) -> String {
        String::from("Finds subdomains using bruteforce.")
    }
}

#[async_trait]
impl SubdomainScanner for BruteForceScan {
    async fn get_subdomains(&self, target: &str) -> Result<Vec<String>, ScanError> {
        log::info!("Getting subdomains by bruteforce trial using words from prefixes.txt..");

        let concurrency: usize = 1000000;
        let file_name = String::from("./brute.txt");

        let subdomains_file = File::open(&file_name)?;
        let reader = BufReader::new(subdomains_file);

        let start_time = Instant::now();

        let (input_tx, input_rx) = mpsc::channel(concurrency);
        let (output_tx, output_rx) = mpsc::channel(concurrency);

        tokio::spawn(async move {
            for line in reader.lines() {
                log::info!("{:?} sending..", &line);
                let _ = input_tx.send(line).await;
            }
        });

        let input_rx_stream = tokio_stream::wrappers::ReceiverStream::new(input_rx);
        input_rx_stream
            .for_each_concurrent(concurrency, |prefix| {
                let output_tx = output_tx.clone();
                let prefix = prefix.unwrap();
                async move {
                    let subdomain = format!("{}.{}", prefix, &target);
                    log::info!("bruteforce subdomain: {}", &subdomain);
                    let _ = output_tx.send(subdomain).await;
                }
            })
            .await;

        //close channel
        drop(output_tx);

        let output_rx_stream = tokio_stream::wrappers::ReceiverStream::new(output_rx);
        let subdomains = output_rx_stream.collect().await;

        let _scan_duration = start_time.elapsed();

        log::info!(
            "\nBruteforce subdomain scan. time elapsed: {:?}",
            _scan_duration
        );
        return Ok(subdomains);
    }
}
