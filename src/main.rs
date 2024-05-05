use anyhow::{Context, Result};
use clap::Parser;
use client::Client;

mod client;

/// quiccat (qc) - A simple CLI tool to test QUIC connectivity
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The endpoint to attempt a connection to. This is in the form `hostname:port`
    /// or `ip:port`
    #[arg()]
    endpoint: String,

    /// The timeout for the connection attempt, in seconds. If not provided, the default
    /// is 2 seconds.
    #[arg(short, long, default_value = "2")]
    timeout: u64,

    /// The path to the trusted CA certificate file. If not provided, the system's
    /// root CA store will be used
    #[arg(short, long)]
    ca_cert_path: Option<String>,

    /// The name of the server that the certificate should be valid for.
    /// If not provided, the server name will be inferred from the provided endpoint
    #[arg(short, long)]
    server_name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse the command line arguments
    let args = Args::parse();

    // Create a new client based on the provided arguments
    let mut config =
        Client::new(&args.ca_cert_path).with_context(|| "Failed to create QUIC client")?;

    // Test the connection to the remote endpoint
    config
        .test_connection(&args.endpoint, args.server_name, args.timeout)
        .await
        .with_context(|| "Connection failed")?;

    Ok(())
}
