use serde::Serialize;

pub trait SerdeExt {
    fn pretty_string(&self) -> String;
}

impl<T> SerdeExt for T
where
    T: ?Sized + Serialize,
{
    /// Converts a serializable struct to pretty json
    /// Fails in case the serde fails to convert it to pretty string.
    fn pretty_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
