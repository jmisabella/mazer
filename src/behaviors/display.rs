use serde::Serialize;
use serde_json;

/// A helper trait that provides JSON conversion methods.
/// Types that implement `Serialize` (or themselves as JSON representations) automatically get these methods.
pub trait JsonDisplay {
    /// Convert the type to a compact JSON string.
    fn to_json(&self) -> Result<String, serde_json::Error>;

    /// Convert the type to a pretty-printed JSON string.
    fn to_pretty_json(&self) -> Result<String, serde_json::Error>;
}

/// Blanket implementation of `JsonDisplay` for all serializable types.
impl<T> JsonDisplay for T
where
    T: Serialize,
{
    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    fn to_pretty_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self)
    }
}
