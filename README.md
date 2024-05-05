## quiccat (qc) - A simple CLI tool to test QUIC connectivity ⚡🐈

## Installation
```bash
cargo install --release --git https://github.com/rob-maron/quiccat
```

## Usage
```bash
qc [OPTIONS] <ENDPOINT>
```

### Examples
Verify connection to a QUIC test connection endpoint:
```bash
qc http3.is:443
```

To a local QUIC server with a 1s timeout:
```bash
qc localhost:4433 -t 1 --ca-cert-path /path/to/ca-cert.pem
```

To a QUIC server with an alternative TLS certificate name indicated:
```bash
qc example.com:443 --server-name example.org
```

### Arguments
  - `ENDPOINT`: The endpoint to attempt a connection to. This is in the form `hostname:port` or `ip:port`.

### Options
  - `-c, --ca-cert-path` `<CA_CERT_PATH>`: The path to the trusted CA certificate file. If not provided, the system's root CA store will be used.

  - `-t, --timeout` `<TIMEOUT>`: The timeout for the connection attempt, in seconds. If not provided, the default is 2 seconds.

  - `-s, --server-name` `<SERVER_NAME>`: The name of the server that the certificate should be valid for. If not provided, the server name will be inferred from the provided endpoint.
  
  - `-h, --help`: Print help

  - `-V, --version`: Print version


## In-progress
- [x] Support for custom certificates
- [x] Support for custom SNI
- [ ] Support for running a test server
- [ ] Telnet-like plaintext mode