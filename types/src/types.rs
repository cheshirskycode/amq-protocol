use crate::value::AMQPValue;

use std::collections::BTreeMap;
use std::fmt;

use serde_derive::{Deserialize, Serialize};

/// Enumeration referencing all the available AMQP types
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum AMQPType {
    /// A bool
    Boolean,
    /// An i8
    ShortShortInt,
    /// A u8
    ShortShortUInt,
    /// An i16
    ShortInt,
    /// A u16
    ShortUInt,
    /// An i32
    LongInt,
    /// A u32
    LongUInt,
    /// An i64
    LongLongInt,
    /// A u64
    LongLongUInt,
    /// An f32
    Float,
    /// An f64
    Double,
    /// A decimal value represented by a scale and a value
    DecimalValue,
    /// Deprecated, a String
    ShortString,
    /// A String
    LongString,
    /// An array of AMQPValue
    FieldArray,
    /// A timestamp (u32)
    Timestamp,
    /// A Map<String, AMQPValue>
    FieldTable,
    /// An array of bytes, RabbitMQ specific
    ByteArray, /* ByteArray is specific to RabbitMQ */
    /// No value
    Void,
}

impl AMQPType {
    /// Get the AMQPType corresponding to the given id.
    /// We don't strictly follow the spec here but rather the RabbitMQ implementation
    /// 's' means ShortInt (like 'U') instead of ShortString
    /// 'l' and 'L' both mean LongLongInt (no LongLongUInt)
    pub fn from_id(id: char) -> Option<AMQPType> {
        match id {
            't' => Some(AMQPType::Boolean),
            'b' => Some(AMQPType::ShortShortInt),
            'B' => Some(AMQPType::ShortShortUInt),
            /* Specs says 'U', RabbitMQ says 's' (which means ShortString in specs) */
            's' |
            'U' => Some(AMQPType::ShortInt),
            'u' => Some(AMQPType::ShortUInt),
            'I' => Some(AMQPType::LongInt),
            'i' => Some(AMQPType::LongUInt),
            /* RabbitMQ treats both 'l' and 'L' as LongLongInt and ignores LongLongUInt */
            'L' |
            'l' => Some(AMQPType::LongLongInt),
            'f' => Some(AMQPType::Float),
            'd' => Some(AMQPType::Double),
            'D' => Some(AMQPType::DecimalValue),
            'S' => Some(AMQPType::LongString),
            'A' => Some(AMQPType::FieldArray),
            'T' => Some(AMQPType::Timestamp),
            'F' => Some(AMQPType::FieldTable),
            'x' => Some(AMQPType::ByteArray),
            'V' => Some(AMQPType::Void),
            _   => None,
        }
    }

    /// Get the id from an AMQPType
    /// We don't strictly follow the spec here but rather the RabbitMQ implementation
    /// ShortString doesn't have an id, we return '_' instead
    /// ShortInt is supposed to be 'U' but we use 's'
    /// LongLongUInt is supposed to be 'L' but we return 'l' as LongLongInt
    pub fn get_id(&self) -> char {
        match *self {
            AMQPType::Boolean        => 't',
            AMQPType::ShortShortInt  => 'b',
            AMQPType::ShortShortUInt => 'B',
            /* Specs says 'U', RabbitMQ says 's' (which means ShortString in specs) */
            AMQPType::ShortInt       => 's',
            AMQPType::ShortUInt      => 'u',
            AMQPType::LongInt        => 'I',
            AMQPType::LongUInt       => 'i',
            /* RabbitMQ treats both 'l' and 'L' as LongLongInt and ignores LongLongUInt */
            AMQPType::LongLongInt    |
            AMQPType::LongLongUInt   => 'l',
            AMQPType::Float          => 'f',
            AMQPType::Double         => 'd',
            AMQPType::DecimalValue   => 'D',
            /* ShortString only exists for internal usage, we shouldn't ever have to use this */
            AMQPType::ShortString    => '_',
            AMQPType::LongString     => 'S',
            AMQPType::FieldArray     => 'A',
            AMQPType::Timestamp      => 'T',
            AMQPType::FieldTable     => 'F',
            AMQPType::ByteArray      => 'x',
            AMQPType::Void           => 'V',
        }
    }
}

impl fmt::Display for AMQPType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// A bool
pub type Boolean        = bool;
/// An i8
pub type ShortShortInt  = i8;
/// A u8
pub type ShortShortUInt = u8;
/// An i16
pub type ShortInt       = i16;
/// A u16
pub type ShortUInt      = u16;
/// An i32
pub type LongInt        = i32;
/// A u32
pub type LongUInt       = u32;
/// An i64
pub type LongLongInt    = i64;
/// A u64
pub type LongLongUInt   = u64;
/// A f32
pub type Float          = f32;
/// A f64
pub type Double         = f64;
/// A String (deprecated)
pub type ShortString    = String;
/// A String
pub type LongString     = String;
/// An array of AMQPValue
pub type FieldArray     = Vec<AMQPValue>;
/// A timestamp (u32)
pub type Timestamp      = LongLongUInt;
/// A Map<String, AMQPValue>
pub type FieldTable     = BTreeMap<ShortString, AMQPValue>;
/// An array of bytes (RabbitMQ specific)
pub type ByteArray      = Vec<u8>;
/// No value
pub type Void           = ();

/// A Decimal value composed of a scale and a value
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct DecimalValue {
    /// The scale of the value
    pub scale: ShortShortUInt,
    /// The actual value
    pub value: LongUInt,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_type_from_id() {
        assert_eq!(AMQPType::from_id('T'), Some(AMQPType::Timestamp));
        assert_eq!(AMQPType::from_id('S'), Some(AMQPType::LongString));
        assert_eq!(AMQPType::from_id('s'), Some(AMQPType::ShortInt));
        assert_eq!(AMQPType::from_id('U'), Some(AMQPType::ShortInt));
        assert_eq!(AMQPType::from_id('l'), Some(AMQPType::LongLongInt));
        assert_eq!(AMQPType::from_id('z'), None);
    }

    #[test]
    fn test_type_get_id() {
        assert_eq!(AMQPType::LongLongInt.get_id(),  'l');
        assert_eq!(AMQPType::LongLongUInt.get_id(), 'l');
        assert_eq!(AMQPType::ShortString.get_id(),  '_');
    }

    #[test]
    fn test_type_to_string() {
        assert_eq!(AMQPType::Boolean.to_string(), "Boolean");
        assert_eq!(AMQPType::Void.to_string(),    "Void");
    }
}
