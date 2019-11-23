use std::fmt;
use serde::{
    ser::{Serialize, Serializer},
    de::{self, Deserialize, Deserializer, Visitor},
};
use super::License;

struct LicenseVisitor;

impl<'de> Visitor<'de> for LicenseVisitor {
    type Value = License<'de>;

    #[inline]
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a license string")
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where E: de::Error,
    {
        Ok(License::owned(v))
    }

    #[inline]
    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
        where E: de::Error,
    {
        Ok(License::from(v))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where E: de::Error,
    {
        Ok(License::owned(v))
    }
}

impl<'de: 'a, 'a> Deserialize<'de> for License<'a> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_str(LicenseVisitor)
    }
}

impl Serialize for License<'_> {
    #[inline]
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(self.id())
    }
}
