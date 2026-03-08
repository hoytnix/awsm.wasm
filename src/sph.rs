use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Pointer(pub String);

impl Pointer {
    pub fn derive_sph(file_path: &str, anchor_id: &str, node_signature: &str, parent_hash: &str) -> Self {
        let mut hasher = blake3::Hasher::new();
        hasher.update(parent_hash.as_bytes());
        hasher.update(node_signature.as_bytes());
        let hash_hex = hasher.finalize().to_hex();
        Pointer(format!("urn:awsm:{}:{}/{}", file_path, anchor_id, &hash_hex[0..12]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_sph() {
        let ptr = Pointer::derive_sph("foo.rs", "anchor", "signature", "parent");
        assert!(ptr.0.starts_with("urn:awsm:foo.rs:anchor/"));
    }
}
