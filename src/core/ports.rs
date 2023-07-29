use super::models::{Port, Subdomain};
use super::ports_list::PORTS_LIST;

use futures::StreamExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc;

use std::net::{SocketAddr, ToSocketAddrs};
use std::time::Duration;

pub async fn scan_ports(concurrency: usize, subdomain: Subdomain) -> Subdomain {
    log::info!("{}. Scanning for open ports:....", &subdomain.domain_name);
    let mut result = subdomain.clone();

    let socket_addresses: Vec<SocketAddr> = format!("{}:1024", subdomain.domain_name)
        .to_socket_addrs()
        .expect("Scanning port...Creating socket address failed")
        .collect();

    if socket_addresses.is_empty() {
        return subdomain;
    }

    let address = socket_addresses[0];

    let (input_tx, input_rx) = mpsc::channel(concurrency);
    let (output_tx, output_rx) = mpsc::channel(concurrency);

    tokio::spawn(async move {
        for port in PORTS_LIST {
            let _ = input_tx.send(*port).await;
        }
    });

    let input_receiver_stream = tokio_stream::wrappers::ReceiverStream::new(input_rx);
    input_receiver_stream
        .for_each_concurrent(concurrency, |port| {
            let output_tx = output_tx.clone();
            async move {
                let port: Port = scan_port(address, port).await;
                if port.conn_open {
                    let _ = output_tx.send(port).await;
                }
            }
        })
        .await;

    drop(output_tx);

    let output_receiver_stream = tokio_stream::wrappers::ReceiverStream::new(output_rx);
    result.open_ports = output_receiver_stream.collect().await;

    result
}

// Using a simple method: attempted tcp connection
async fn scan_port(mut socket_address: SocketAddr, port: u16) -> Port {
    let timeout = Duration::from_secs(3);
    let mut is_open = false;

    socket_address.set_port(port);

    if tokio::time::timeout(timeout, TcpStream::connect(&socket_address))
        .await
        .is_ok()
    {
        is_open = true;
    }

    Port {
        port,
        conn_open: is_open,
    }
}
