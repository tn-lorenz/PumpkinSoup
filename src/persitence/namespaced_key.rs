pub struct NamespacedKey {
    namespace: String,
    key: String,
}

impl NamespacedKey {
    pub fn new(namespace: &str, key: &str) -> Self {
        Self {
            namespace: namespace.to_ascii_lowercase(),
            key: key.to_string(),
        }
    }
}

#[macro_export]
macro_rules! ns_key {
    ($value:expr) => {
        $crate::NamespacedKey::new(env!("CARGO_PKG_NAME"), $value)
    };
}
