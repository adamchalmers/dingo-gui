use message::Message;
use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket},
    time::Duration,
};

use crate::dns::dns_types::RecordType;

mod dns_types;
mod header;
mod message;
mod parse;
mod parser_utils;
mod question;
mod record;

pub(crate) const MAX_UDP_BYTES: usize = 512;

pub fn resolve(hostname: &str) -> anyhow::Result<Vec<String>> {
    let query_id = {
        use rand::Rng;
        rand::thread_rng().gen()
    };

    assert!(
        hostname.ends_with('.'),
        "DNS names must end with a period, but {} does not",
        hostname
    );

    let request = Message::new_query(query_id, hostname, RecordType::A)?;
    let default_resolver = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(1, 1, 1, 1), 53));
    let verbose = true;
    let (binary_response, bytes_received) = send_request(request, default_resolver, verbose)?;
    let payload = binary_response[..bytes_received].to_vec();
    let response = Message::deserialize(payload)?;

    let records: Vec<_> = response
        .answer
        .into_iter()
        .map(|record| record.as_dns_response())
        .collect();
    println!("Returning {} IPs for {hostname}", records.len());
    Ok(records)
}

fn send_request(
    req: Message,
    resolver: SocketAddr,
    verbose: bool,
) -> anyhow::Result<(Vec<u8>, usize)> {
    use anyhow::{anyhow, bail};
    // Connect to the DNS resolver
    let local_addr = "0.0.0.0:0";
    let socket = UdpSocket::bind(local_addr).expect("couldn't bind to a local address");
    socket.set_read_timeout(Some(Duration::from_secs(5)))?;
    if verbose {
        println!("Bound to local {}", socket.local_addr()?);
    }
    socket
        .connect(resolver)
        .expect("couldn't connect to the DNS resolver");
    if verbose {
        println!("Connected to remote {resolver}");
    }

    // Send the DNS resolver the message
    let body = req.serialize_bytes()?;
    if verbose {
        println!("Request size: {} bytes", body.len());
    }
    let bytes_sent = socket.send_to(&body, resolver).expect("couldn't send data");
    if bytes_sent != body.len() {
        bail!("Only {bytes_sent} bytes, message was probably truncated");
    }

    // Get the resolver's response.
    // Note, you have to actually allocate space to write into.
    // I was originally using an empty vector, but reading into an empty vector always
    // instantly succeeds (by writing nothing), so I was discarding the response.
    // See <https://users.rust-lang.org/t/empty-response-from-udp-recv-w-tokio-and-futures/20241/2>
    let mut response_buf = vec![0; MAX_UDP_BYTES];
    match socket.recv_from(&mut response_buf) {
        Ok((received, remote_addr)) => {
            assert_eq!(remote_addr, resolver, "The DNS response came from {remote_addr} but I expected it to come from the resolver at {resolver}");
            Ok((response_buf, received))
        }
        Err(e) => Err(anyhow!("recv function failed: {:?}", e)),
    }
}
