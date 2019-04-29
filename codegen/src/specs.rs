use crate::internal::*;

use amq_protocol_types::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};

use std::collections::BTreeMap;

/// Structure holding the definition of the protocol
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQProtocolDefinition {
    /// The name of the protocol
    pub name:          ShortString,
    /// The major protocol version
    pub major_version: ShortShortUInt,
    /// The minor protocol version
    pub minor_version: ShortShortUInt,
    /// The revision of the protocol version
    pub revision:      ShortShortUInt,
    /// The default port of the protocol
    pub port:          LongUInt,
    /// The copyright holder of the protocol specification
    pub copyright:     LongString,
    /// The domains defined by the protocol specification
    pub domains:       BTreeMap<ShortString, AMQPType>,
    /// The constants defined by the protocol specification
    pub constants:     Vec<AMQPConstant>,
    /// The soft errors defined by the protocol specification
    pub soft_errors:   Vec<AMQPConstant>,
    /// The hard errors defined by the protocol specification
    pub hard_errors:   Vec<AMQPConstant>,
    /// The classes defined by the protocol specification
    pub classes:       Vec<AMQPClass>,
}

impl AMQProtocolDefinition {
    /// Load protocol definition from reference specification
    pub fn load(metadata: Option<Value>) -> AMQProtocolDefinition {
        let specs = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/specs/amqp-rabbitmq-0.9.1.json"));

        from_str::<_AMQProtocolDefinition>(specs).expect("Failed to parse AMQP specs file").into_specs(&metadata.unwrap_or_default())
    }
}

/// A constant as defined in the AMQP specification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPConstant {
    /// The name of the constant
    pub name:      ShortString,
    /// The value of the constant
    pub value:     ShortUInt,
    /// The type of the constant
    #[serde(rename="type")]
    pub amqp_type: AMQPType,
}

/// A class as defined in the AMQP specification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPClass {
    /// The id of the class
    pub id:             ShortUInt,
    /// The methods of the class
    pub methods:        Vec<AMQPMethod>,
    /// The name of the class
    pub name:           ShortString,
    /// The properties of the class
    pub properties:     Vec<AMQPProperty>,
    /// Extra metadata for code generation
    pub metadata:       Value,
}

/// A method as defined in the AMQP specification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPMethod {
    /// The id of the method
    pub id:            ShortUInt,
    /// The arguments of the method
    pub arguments:     Vec<AMQPArgument>,
    /// The name of the method
    pub name:          ShortString,
    /// Whether this method is synchronous or not
    pub synchronous:   Boolean,
    /// Extra metadata for code generation
    pub metadata:      Value,
    /// Whether this method is a reply or not
    pub is_reply:      bool,
}

/// An argument as defined in the AMQP specification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum AMQPArgument {
    /// The argument is holding a value
    Value(AMQPValueArgument),
    /// The argument is holding flags
    Flags(Vec<AMQPFlagArgument>),
}

/// An argument holding a value as defined in the AMQP specification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPValueArgument {
    /// The type of the argument's value
    #[serde(rename="type")]
    pub amqp_type:     AMQPType,
    /// The name of the argument's value
    pub name:          ShortString,
    /// The default value of the argument's value
    pub default_value: Option<AMQPValue>,
    /// The domain of the argument's value
    pub domain:        Option<ShortString>,
    /// Whether the default value is forced or not
    pub force_default: bool,
}

/// An argument holding flags as defined in the AMQP specification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPFlagArgument {
    /// The name of the flag
    pub name:          ShortString,
    /// The default value for the flag
    pub default_value: Boolean,
}

/// A property as defined in the AMQP specification
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AMQPProperty {
    /// The type of the property
    #[serde(rename="type")]
    pub amqp_type: AMQPType,
    /// The name of the property
    pub name:      ShortString,
}
