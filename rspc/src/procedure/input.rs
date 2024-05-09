use serde::de::DeserializeOwned;

use super::InputValue;

pub trait Input: Sized {
    fn from_value(value: InputValue<Self>) -> Result<Self, ()>;
}

impl<T: DeserializeOwned + 'static> Input for T {
    fn from_value(value: InputValue<Self>) -> Result<Self, ()> {
        Ok(value.deserialize()?)
    }
}