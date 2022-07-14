//! For `Option<http::*>`
//!
//! ## Usage
//!
//! You must annotate fields with `#[serde(with = "http_serde::option::<appropriate method>")]`.
//!
//! ```rust
//! use http::*;
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyStruct {
//!     #[serde(with = "http_serde::option::header_map")]
//!     headers: Option<HeaderMap>,
//!
//!     #[serde(with = "http_serde::option::status_code")]
//!     status: Option<StatusCode>,
//!
//!     #[serde(with = "http_serde::option::method")]
//!     method: Option<Method>,
//!
//!     #[serde(with = "http_serde::option::uri")]
//!     uri: Option<Uri>,
//!
//!     #[serde(with = "http_serde::option::authority")]
//!     authority: Option<uri::Authority>,
//!
//!     #[serde(with = "http_serde::option::version")]
//!     version: Option<Version>,
//! }
//! ```

use serde::de::{Error, MapAccess, Visitor};
use serde::{Deserializer, Serializer};
use std::fmt;
use std::fmt::Formatter;

macro_rules! impl_visit {
    ($name: ident, $type: ty) => {
        fn $name<E>(self, v: $type) -> Result<Self::Value, E> where E: Error {
            self.0.$name(v).map(|v| Some(v))
        }
    }
}

struct OptionVisitor<V>(V);
impl<'de, T, V> Visitor<'de> for OptionVisitor<V>
where
    V: Visitor<'de, Value = T>,
{
    type Value = Option<T>;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "valid option")
    }

    fn visit_map<M>(self, access: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
    {
        self.0.visit_map(access).map(|v| Some(v))
    }

    impl_visit!(visit_i32, i32);
    impl_visit!(visit_i16, i16);
    impl_visit!(visit_u8, u8);
    impl_visit!(visit_u32, u32);
    impl_visit!(visit_i64, i64);
    impl_visit!(visit_u64, u64);
    impl_visit!(visit_u16, u16);
    impl_visit!(visit_str, &str);
    impl_visit!(visit_string, String);
}

macro_rules! impl_option_with {
    ($name: ident, $value: ty, $visitor: expr) => {
        /// For `Option<$value>`
        ///
        /// `#[serde(with = "http_serde::option::$name")]`
        pub mod $name {
            use super::*;

            pub fn serialize<S: Serializer>(v: &Option<$value>, ser: S) -> Result<S::Ok, S::Error> {
                match v {
                    Some(v) => crate::$name::serialize(v, ser),
                    None => ser.serialize_none(),
                }
            }

            pub fn deserialize<'de, D>(de: D) -> Result<Option<$value>, D::Error>
                where
                    D: Deserializer<'de>,
            {
                let is_human_readable = de.is_human_readable();
                de.deserialize_option(OptionVisitor($visitor(is_human_readable)))
            }
        }
    }
}

impl_option_with!(header_map, http::HeaderMap, |is_human_readable: bool| crate::header_map::HeaderMapVisitor { is_human_readable });
impl_option_with!(status_code, http::StatusCode, |_| crate::status_code::StatusVisitor);
impl_option_with!(method, http::Method, |_| crate::method::MethodVisitor);
impl_option_with!(uri, http::Uri, |_| crate::uri::UriVisitor);
impl_option_with!(authority, http::uri::Authority, |_| crate::authority::AuthorityVisitor);
impl_option_with!(version, http::Version, |_| crate::version::VersionVisitor);
