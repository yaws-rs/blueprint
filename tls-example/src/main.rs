//! TLS Blueprint Example
// Stack:
// - TLS
// - Transformer as follows:
// HELLO TRANSFORM                - Left In
// I AM TRANSFORMER               - Left Out
// AGREED LET'S TRANSFORM         - Left In
// 1111 2222 3333 4444 5555 6666  - Left In, Right Out
// 2222 3333 4444 5555 6666 7777  - Right In, Left Out
// 3333 5555 7777 9999 1111 1313  - Left In


use blueprint::{BluePrint, Orbit};
use blueprint_tls::{TlsClient, TlsServer};
use blueprint_tls::{TlsClientConfig, TlsServerConfig};

const CA: &'static str = "../../tls/blueprint/certs/ca.rsa4096.crt";
const CERT: &'static str = "../../tls/blueprint/certs/rustcryp.to.rsa4096.ca_signed.crt";
const KEY: &'static str = "../../tls/blueprint/certs/rustcryp.to.rsa4096.key";

use std::path::Path;

fn main() {
    let config_server =
        TlsServerConfig::with_certs_and_key_file(Path::new(CA), Path::new(CERT), Path::new(KEY))
            .unwrap();
    let config_client = TlsClientConfig::with_hostname("localhost").unwrap();

    let mut server = TlsServer::with_config(config_server).unwrap();
    let mut client = TlsClient::with_config(config_client).unwrap();

    struct Empty;
    let mut u = Empty;
    
}
