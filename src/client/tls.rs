use std::{sync::Arc, time::SystemTime};

use rustls::{client::ServerCertVerified, Certificate, ClientConfig, ServerName};

/// This struct implements the `ServerCertVerifier` trait from `rustls` to skip server
/// certificate verification (used with the `-i, --insecure` flag)
pub struct SkipServerCertVerify;

/// Create a new `ClientConfig` with the `SkipServerCertVerify` verifier
impl SkipServerCertVerify {
    pub fn new_config() -> ClientConfig {
        // Create a new `ClientConfig` defaults and the custom certificate verifier
        rustls::ClientConfig::builder()
            .with_safe_defaults()
            .with_custom_certificate_verifier(Arc::from(Self {}))
            .with_no_client_auth()
    }
}

/// Implement the `ServerCertVerifier` trait for `SkipServerCertVerify` so we can
/// use it in the client
impl rustls::client::ServerCertVerifier for SkipServerCertVerify {
    fn verify_server_cert(
        &self,
        _end_entity: &Certificate,
        _intermediates: &[Certificate],
        _server_name: &ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: SystemTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        // Skip server certificate verification
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}
