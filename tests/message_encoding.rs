#[macro_use]
extern crate proto_derive;

extern crate proto;
extern crate bytes;

#[macro_use]
extern crate log;
extern crate env_logger;

use std::io::Cursor;

use bytes::Buf;

use proto::Message;

// Creates a checker function for each field trait.
fn check_message<M>(msg: M) where M: Message + PartialEq {
    let expected_len = msg.encoded_len();

    let mut buf = Vec::with_capacity(18);
    msg.encode(&mut buf).unwrap();

    assert_eq!(expected_len, buf.len());

    info!("encoded message: {:?}", buf);

    let mut buf = Cursor::new(&mut buf).take(expected_len);
    let roundtrip = M::decode(&mut buf).unwrap();

    if buf.has_remaining() {
        panic!(format!("expected buffer to be empty: {}", buf.remaining()));
    }

    assert_eq!(msg, roundtrip);
}

#[derive(Clone, Debug, PartialEq, Message)]
pub struct RepeatedFloats {
    #[proto(float, tag="11")]
    pub single_float: f32,
    #[proto(float, repeated, packed="true", tag="41")]
    pub repeated_float: Vec<f32>,
}

#[test]
fn check_repeated_floats() {
    let _ = env_logger::init();
    check_message(RepeatedFloats {
        single_float: 0.0,
        repeated_float: vec![
            0.1,
            340282300000000000000000000000000000000.0,
            0.000000000000000000000000000000000000011754944
        ],
    });
}

#[test]
fn check_scalar_types() {
    let _ = env_logger::init();
    let scalar_types = ScalarTypes::default();
    check_message(scalar_types);
}

/// A protobuf message which contains all scalar types.
#[derive(Clone, Debug, PartialEq, Message)]
pub struct ScalarTypes {
    #[proto(int32, tag="001")]
    pub int32: i32,
    #[proto(int64, tag="002")]
    pub int64: i64,
    #[proto(uint32, tag="003")]
    pub uint32: u32,
    #[proto(uint64, tag="004")]
    pub uint64: u64,
    #[proto(sint32, tag="005")]
    pub sint32: i32,
    #[proto(sint64, tag="006")]
    pub sint64: i64,
    #[proto(fixed32, tag="007")]
    pub fixed32: u32,
    #[proto(fixed64, tag="008")]
    pub fixed64: u64,
    #[proto(sfixed32, tag="009")]
    pub sfixed32: i32,
    #[proto(sfixed64, tag="010")]
    pub sfixed64: i64,
    #[proto(float, tag="011")]
    pub float: f32,
    #[proto(double, tag="012")]
    pub double: f64,
    #[proto(bool, tag="013")]
    pub _bool: bool,
    #[proto(string, tag="014")]
    pub string: String,
    #[proto(bytes, tag="015")]
    pub bytes: Vec<u8>,

    #[proto(int32, required, tag="101")]
    pub required_int32: i32,
    #[proto(int64, required, tag="102")]
    pub required_int64: i64,
    #[proto(uint32, required, tag="103")]
    pub required_uint32: u32,
    #[proto(uint64, required, tag="104")]
    pub required_uint64: u64,
    #[proto(sint32, required, tag="105")]
    pub required_sint32: i32,
    #[proto(sint64, required, tag="106")]
    pub required_sint64: i64,
    #[proto(fixed32, required, tag="107")]
    pub required_fixed32: u32,
    #[proto(fixed64, required, tag="108")]
    pub required_fixed64: u64,
    #[proto(sfixed32, required, tag="109")]
    pub required_sfixed32: i32,
    #[proto(sfixed64, required, tag="110")]
    pub required_sfixed64: i64,
    #[proto(float, required, tag="111")]
    pub required_float: f32,
    #[proto(double, required, tag="112")]
    pub required_double: f64,
    #[proto(bool, required, tag="113")]
    pub required_bool: bool,
    #[proto(string, required, tag="114")]
    pub required_string: String,
    #[proto(bytes, required, tag="115")]
    pub required_bytes: Vec<u8>,

    #[proto(int32, optional, tag="201")]
    pub optional_int32: Option<i32>,
    #[proto(int64, optional, tag="202")]
    pub optional_int64: Option<i64>,
    #[proto(uint32, optional, tag="203")]
    pub optional_uint32: Option<u32>,
    #[proto(uint64, optional, tag="204")]
    pub optional_uint64: Option<u64>,
    #[proto(sint32, optional, tag="205")]
    pub optional_sint32: Option<i32>,
    #[proto(sint64, optional, tag="206")]
    pub optional_sint64: Option<i64>,

    #[proto(fixed32, optional, tag="207")]
    pub optional_fixed32: Option<u32>,
    #[proto(fixed64, optional, tag="208")]
    pub optional_fixed64: Option<u64>,
    #[proto(sfixed32, optional, tag="209")]
    pub optional_sfixed32: Option<i32>,
    #[proto(sfixed64, optional, tag="210")]
    pub optional_sfixed64: Option<i64>,
    #[proto(float, optional, tag="211")]
    pub optional_float: Option<f32>,
    #[proto(double, optional, tag="212")]
    pub optional_double: Option<f64>,
    #[proto(bool, optional, tag="213")]
    pub optional_bool: Option<bool>,
    #[proto(string, optional, tag="214")]
    pub optional_string: Option<String>,
    #[proto(bytes, optional, tag="215")]
    pub optional_bytes: Option<Vec<u8>>,

    #[proto(int32, repeated, packed="false", tag="301")]
    pub repeated_int32: Vec<i32>,
    #[proto(int64, repeated, packed="false", tag="302")]
    pub repeated_int64: Vec<i64>,
    #[proto(uint32, repeated, packed="false", tag="303")]
    pub repeated_uint32: Vec<u32>,
    #[proto(uint64, repeated, packed="false", tag="304")]
    pub repeated_uint64: Vec<u64>,
    #[proto(sint32, repeated, packed="false", tag="305")]
    pub repeated_sint32: Vec<i32>,
    #[proto(sint64, repeated, packed="false", tag="306")]
    pub repeated_sint64: Vec<i64>,
    #[proto(fixed32, repeated, packed="false", tag="307")]
    pub repeated_fixed32: Vec<u32>,
    #[proto(fixed64, repeated, packed="false", tag="308")]
    pub repeated_fixed64: Vec<u64>,
    #[proto(sfixed32, repeated, packed="false", tag="309")]
    pub repeated_sfixed32: Vec<i32>,
    #[proto(sfixed64, repeated, packed="false", tag="310")]
    pub repeated_sfixed64: Vec<i64>,
    #[proto(float, repeated, packed="false", tag="311")]
    pub repeated_float: Vec<f32>,
    #[proto(double, repeated, packed="false", tag="312")]
    pub repeated_double: Vec<f64>,
    #[proto(bool, repeated, packed="false", tag="313")]
    pub repeated_bool: Vec<bool>,
    #[proto(string, repeated, packed="false", tag="315")]
    pub repeated_string: Vec<String>,
    #[proto(bytes, repeated, packed="false", tag="316")]
    pub repeated_bytes: Vec<Vec<u8>>,

    #[proto(int32, repeated, tag="401")]
    pub packed_int32: Vec<i32>,
    #[proto(int64, repeated, tag="402")]
    pub packed_int64: Vec<i64>,
    #[proto(uint32, repeated, tag="403")]
    pub packed_uint32: Vec<u32>,
    #[proto(uint64, repeated, tag="404")]
    pub packed_uint64: Vec<u64>,
    #[proto(sint32, repeated, tag="405")]
    pub packed_sint32: Vec<i32>,
    #[proto(sint64, repeated, tag="406")]
    pub packed_sint64: Vec<i64>,
    #[proto(fixed32, repeated, tag="407")]
    pub packed_fixed32: Vec<u32>,

    #[proto(fixed64, repeated, tag="408")]
    pub packed_fixed64: Vec<u64>,
    #[proto(sfixed32, repeated, tag="409")]
    pub packed_sfixed32: Vec<i32>,
    #[proto(sfixed64, repeated, tag="410")]
    pub packed_sfixed64: Vec<i64>,
    #[proto(float, repeated, tag="411")]
    pub packed_float: Vec<f32>,
    #[proto(double, repeated, tag="412")]
    pub packed_double: Vec<f64>,
    #[proto(bool, repeated, tag="413")]
    pub packed_bool: Vec<bool>,
    #[proto(string, repeated, tag="415")]
    pub packed_string: Vec<String>,
    #[proto(bytes, repeated, tag="416")]
    pub packed_bytes: Vec<Vec<u8>>,
}

/// A protobuf message with default value.
#[derive(Clone, Debug, PartialEq, Message)]
pub struct DefaultValues {
    #[proto(int32, tag="1", default="42")]
    pub int32: i32,

    #[proto(int32, optional, tag="2", default="88")]
    pub optional_int32: Option<i32>,

    #[proto(string, tag="3", default="fourty two")]
    pub string: String,

    #[proto(enumeration="BasicEnumeration", tag="4", default="ONE")]
    pub enumeration: i32,

    #[proto(enumeration="BasicEnumeration", optional, tag="5", default="TWO")]
    pub optional_enumeration: Option<i32>,

    #[proto(enumeration="BasicEnumeration", repeated, tag="6")]
    pub repeated_enumeration: Vec<i32>,
}

#[test]
fn check_default_values() {
    let default = DefaultValues::default();
    assert_eq!(default.int32, 42);
    assert_eq!(default.optional_int32, None);
    assert_eq!(&default.string, "fourty two");
    assert_eq!(default.enumeration, BasicEnumeration::ONE as i32);
    assert_eq!(default.optional_enumeration, None);
    assert_eq!(&default.repeated_enumeration, &[]);
    assert_eq!(0, default.encoded_len());
}


/// A protobuf enum.
#[derive(Clone, Copy, Debug, PartialEq, Enumeration)]
pub enum BasicEnumeration {
    ZERO = 0,
    ONE = 1,
    TWO = 2,
    THREE = 3,
}

#[derive(Clone, Debug, PartialEq, Message)]
pub struct Basic {
    #[proto(int32, tag="1")]
    pub int32: i32,

    #[proto(bool, repeated, packed="false", tag="2")]
    pub bools: Vec<bool>,

    #[proto(string, tag="3")]
    pub string: String,

    #[proto(string, optional, tag="4")]
    pub optional_string: Option<String>,

    #[proto(enumeration="BasicEnumeration", tag="5")]
    pub enumeration: i32,

    #[proto(map="int32, enumeration(BasicEnumeration)", tag="6")]
    pub enumeration_map: ::std::collections::HashMap<i32, i32>,

    #[proto(map="string, string", tag="7")]
    pub string_map: ::std::collections::HashMap<String, String>,

    #[proto(oneof="BasicOneof", tags="8, 9")]
    pub oneof: Option<BasicOneof>,
}

#[derive(Clone, Debug, PartialEq, Message)]
pub struct Compound {
    #[proto(message, optional, tag="1")]
    pub optional_message: Option<Basic>,

    #[proto(message, required, tag="2")]
    pub required_message: Basic,

    #[proto(message, repeated, tag="3")]
    pub repeated_message: Vec<Basic>,

    #[proto(map="sint32, message", tag="4")]
    pub message_map: ::std::collections::HashMap<i32, Basic>,
}

#[derive(Clone, Debug, PartialEq, Oneof)]
pub enum BasicOneof {
    #[proto(int32, tag="8")]
    Int(i32),
    #[proto(string, tag="9")]
    String(String),
}