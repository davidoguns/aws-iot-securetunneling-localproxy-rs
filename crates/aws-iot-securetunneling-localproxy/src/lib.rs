//! AWS IoT Secure Tunneling Local Proxy Library
//!
//! Provides core functionality for establishing and managing secure tunnels
//! with the AWS IoT Secure Tunneling service.

/// Returns the library version.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

// Include generated source. This is not referencing OUT_DIR. Since we aren't
// changing the generated source per build, this is acceptable, much simpler,
// and kinder to various dev tools to check it in.
pub mod proto {
    include!("./com.amazonaws.iot.securedtunneling.rs");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!version().is_empty());
    }

    #[test]
    fn test_proto_message() {
        //basic check
        use proto::Message;
        use proto::message::Type;

        let mut m = Message {
            // r#type: proto::message::Type::from_str_name("StreamStart").unwrap(),
            r#type: proto::message::Type::StreamStart as i32,
            stream_id: 2,
            ignorable: false,
            payload: vec![0x12u8, 0xffu8],
            service_id: "".to_string(),
            available_service_ids: vec!["SSH".to_string()],
            connection_id: 0,
        };

        assert_eq!(m.stream_id, 2);
        assert_eq!(m.service_id, "".to_string());
        assert_eq!(m.ignorable, false);
    }
}
