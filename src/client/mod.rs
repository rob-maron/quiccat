use std::{net::ToSocketAddrs, sync::Arc, time::Duration};

use anyhow::{Context, Result};
use quinn::{ClientConfig, Endpoint};
use rustls::{Certificate, RootCertStore};
use tokio::time::timeout;

use self::tls::SkipServerCertVerify;

mod tls;

pub struct Client {
    local_endpoint: Endpoint,
}

impl Client {
    /// Create a new QUIC client from the provided CA cert path
    pub fn new(ca_cert_path: &Option<String>, insecure: bool) -> Result<Self> {
        // Create a client configuration based on the provided CA cert path and insecure flag.
        //
        // If a cert is not provided, the system's root CA store will be used. If the insecure
        // flag is set, a custom verifier will be used to skip server certificate verification.
        let client_config = if insecure {
            // If the insecure flag is set, create a new client config with a custom verifier
            ClientConfig::new(Arc::from(SkipServerCertVerify::new_config()))
        } else {
            if let Some(ca_cert_path) = ca_cert_path {
                // Create a new root cert store
                let mut cert_store = RootCertStore::empty();

                // Read the CA cert file
                let ca_cert_file = std::fs::read_to_string(ca_cert_path)
                    .with_context(|| "Failed to read CA cert file")?;

                // Parse the CA cert file
                let ca_cert = pem::parse(ca_cert_file.as_bytes())
                    .map_err(|e| anyhow::anyhow!("Failed to parse CA cert: {}", e))?;

                // Add the CA cert to the root cert store
                cert_store
                    .add(&Certificate(ca_cert.into_contents()))
                    .with_context(|| "Failed to add provided CA cert to root cert store")?;

                // Create a new client config with the root cert store
                ClientConfig::with_root_certificates(cert_store)
            } else {
                // Use the system's root cert store
                ClientConfig::with_native_roots()
            }
        };

        // Create a new QUIC client with the provided client config
        let mut local_endpoint = Endpoint::client("0.0.0.0:0".parse()?)
            .with_context(|| "Failed to create QUIC client")?;
        local_endpoint.set_default_client_config(client_config);

        Ok(Self { local_endpoint })
    }

    /// Test a connection to the provided remote endpoint, using the server name if provided
    pub async fn test_connection(
        &mut self,
        remote_endpoint: &str,
        server_name: Option<String>,
        timeout_duration: u64,
    ) -> Result<()> {
        // Resolve the remote endpoint address
        let remote_address = remote_endpoint
            .to_socket_addrs()
            .with_context(|| "Failed to resolve remote endpoint")?
            .filter(|addr| addr.is_ipv4())
            .next()
            .ok_or(anyhow::anyhow!(
                "Did not find suitable address for endpoint"
            ))?;

        // Conditionally set the server name based on the provided argument
        let server_name = if let Some(server_name) = server_name {
            server_name
        } else {
            remote_endpoint
                .split(":")
                .next()
                .ok_or(anyhow::anyhow!("Failed to infer server name from endpoint"))?
                .to_string()
        };

        // Connect to the remote endpoint with a timeout of 2 seconds
        timeout(
            Duration::from_secs(timeout_duration),
            self.local_endpoint
                .connect(remote_address, &server_name)
                .with_context(|| "Failed to configure connection")?,
        )
        .await
        .with_context(|| "Timed out connecting to remote endpoint")?
        .with_context(|| "Failed to connect to remote endpoint")?;

        println!("Connection to {} succeeded!", remote_endpoint);

        Ok(())
    }
}
