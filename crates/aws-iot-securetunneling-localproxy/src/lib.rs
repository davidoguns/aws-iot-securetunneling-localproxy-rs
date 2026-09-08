//! AWS IoT Secure Tunneling Local Proxy Library
//!
//! Provides core functionality for establishing and managing secure tunnels
//! with the AWS IoT Secure Tunneling service.

/// Returns the library version.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!version().is_empty());
    }
}
