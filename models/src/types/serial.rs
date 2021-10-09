use std::ops::{Deref, DerefMut};

#[cfg(feature = "serde")]
use serde::*;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Serial(pub i32);

impl<T> From<T> for Serial
where
    T: Into<i32>,
{
    fn from(obj: T) -> Self {
        Self(obj.into())
    }
}

impl Deref for Serial {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Serial {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &self.0
    }
}

impl AsMut<i32> for Serial {
    fn as_mut(&mut self) -> &mut i32 {
        &mut self.0
    }
}

impl AsRef<i32> for Serial {
    fn as_ref(&self) -> &i32 {
        &self.0
    }
}


impl<DB> Type<DB> for JsonValue
where
    Json<Self>: Type<DB>,
    DB: Database,
{
    fn type_info() -> DB::TypeInfo {
        <Json<Self> as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <Json<Self> as Type<DB>>::compatible(ty)
    }
}


impl<DB> Type<DB> for Vec<JsonValue>
where
    Vec<Json<JsonValue>>: Type<DB>,
    DB: Database,
{
    fn type_info() -> DB::TypeInfo {
        <Vec<Json<JsonValue>> as Type<DB>>::type_info()
    }

    fn compatible(ty: &DB::TypeInfo) -> bool {
        <Vec<Json<JsonValue>> as Type<DB>>::compatible(ty)
    }
}
