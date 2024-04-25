use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use clap::{Arg, Command};
use rand;
use trust_dns::op::{Message, MessageType, OpCode, Query};
use trust_dns::rr::domain::Name;
use trust_dns::rr::record_type::RecordType;
use trust_dns::serialize::binary::*;

fn main() {
    let matches = Command::new("resolve DNS")
        .about("A simple to use DNS resolver")
        .version("1.0.0")
        .arg(
            Arg::new("hostname")
                .help("URL hostname")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("dns-provider")
                .short('d')
                .long("provider")
                .help("Default DNS Provider: 1.1.1.1")
                .required(false),
        )
        .get_matches();

    let hostname = matches.get_one::<String>("hostname").unwrap();
    println!("Value for name: {hostname}");
    let domain_name = Name::from_ascii(hostname).unwrap();
    println!("Domain for name: {:?}", domain_name);

    let default_dns_server = String::from("1.1.1.1");

    let dns_provider_raw = matches
        .get_one::<String>("dns-provider")
        .unwrap_or_else(|| &default_dns_server);
    println!("dns_provider_raw: {dns_provider_raw}");

    let dns_server: SocketAddr = format!("{}:53", dns_provider_raw)
        .parse()
        .expect("invalid address");

    println!("DNS server: {dns_server}");

    let mut request_as_bytes: Vec<u8> = Vec::with_capacity(512);
    let mut response_as_bytes: Vec<u8> = vec![0; 512];

    let mut msg = Message::new();
    msg.set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query)
        .add_query(Query::query(domain_name, RecordType::A))
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    let mut encoder = BinEncoder::new(&mut request_as_bytes);
    msg.emit(&mut encoder).unwrap();

    let localhost = UdpSocket::bind("0.0.0.0:0").expect("cannot bind to local socket");
    let timeout = Duration::from_secs(3);
    localhost.set_read_timeout(Some(timeout)).unwrap();
    localhost.set_nonblocking(false).unwrap();

    let _amt = localhost
        .send_to(&request_as_bytes, &dns_server)
        .expect("Socket misconfigured");

    let (_amt, _remote) = localhost
        .recv_from(&mut response_as_bytes)
        .expect("Timeout reached");

    let dns_message = Message::from_vec(&response_as_bytes).expect("unable to parse response");
    for answer in dns_message.answers() {
        if answer.record_type() == RecordType::A {
            let resource = answer.rdata();
            let ip = resource.to_ip_addr().expect("invalid IP address received");
            println!("{ip}");
        }
    }
}
