//! # FitParser
//!
//! `fitparser` is a utility to parse an ANT FIT file based on a given profile into a more
//! useful form for consuming applications. To that end the [serde](https://github.com/serde-rs/serde)
//! framework is used to allow the data to be serialized into any format supported by serde. This
//! library currently does not support writing FIT files.
//!
//! ## Example
//! Open a file or pass in any other object that implements the Read
//! trait. A vector of data records is returned if deserialization is
//! successful. See the `fit_to_json` example for a command line utility
//! that parses FIT files and exports them as JSON.
//! ```
//! use fitparser;
//! use std::fs::File;
//! use std::io::prelude::*;
//!
//! println!("Parsing FIT files using Profile version: {:?}", fitparser::profile::VERSION);
//! let mut fp = File::open("tests/fixtures/Activity.fit")?;
//! for data in fitparser::from_reader(&mut fp)? {
//!     // print the data in FIT file
//!     println!("{:#?}", data);
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
#![warn(missing_docs)]
use chrono::{DateTime, Local};
use serde::Serialize;
use std::convert;
use std::fmt;

pub mod de;
mod error;
pub mod profile;

pub use de::{from_bytes, from_reader};
pub use error::{Error, ErrorKind, Result};

/// Defines a set of data derived from a FIT Data message.
#[derive(Clone, Debug, Serialize)]
pub struct FitDataRecord {
    /// The kind of message the data came from, the FIT profile defines several messages and
    /// custom messages can be defined by altering the profile
    kind: profile::MesgNum,
    /// All the fields present in this message, a record may not have every possible field defined
    fields: Vec<FitDataField>,
}

impl FitDataRecord {
    /// Create an empty data record with a given kind
    pub fn new(kind: profile::MesgNum) -> Self {
        FitDataRecord {
            kind,
            fields: Vec::new(),
        }
    }

    /// Return the kind of FitDataRecord, this value is defined by the FIT profile.
    pub fn kind(&self) -> profile::MesgNum {
        self.kind
    }

    /// Get all fields as a slice
    pub fn fields(&self) -> &[FitDataField] {
        &self.fields
    }

    /// Add a field to the record
    pub fn push(&mut self, field: FitDataField) {
        self.fields.push(field)
    }

    /// Consume the record and return the field vector for further processing
    pub fn into_vec(self) -> Vec<FitDataField> {
        self.fields
    }
}

/// Stores a value and it's defined units which are set by the FIT profile during decoding
#[derive(Clone, Debug, Serialize)]
pub struct FitDataField {
    name: String,
    number: u8,
    value: Value,
    units: String,
}

impl FitDataField {
    /// Create a new FitDataField
    pub fn new(name: String, number: u8, value: Value, units: String) -> Self {
        FitDataField {
            name,
            number,
            value,
            units,
        }
    }

    /// Return the field name as defined in the FIT profile
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the field definition number
    pub fn number(&self) -> u8 {
        self.number
    }

    /// Return a reference to the stored value
    pub fn value(&self) -> &Value {
        &self.value
    }

    /// Return units associated with the value
    pub fn units(&self) -> &str {
        &self.units
    }

    /// Consume the field and return the value
    pub fn into_value(self) -> Value {
        self.value
    }
}

impl fmt::Display for FitDataField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.units.is_empty() {
            write!(f, "{}", self.value)
        } else {
            write!(f, "{} {}", self.value, self.units)
        }
    }
}

/// Contains arbitrary data in the defined format.
#[derive(Clone, Debug, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum Value {
    /// Timestamp field converted to the local timezone
    Timestamp(DateTime<Local>),
    /// Unsigned 8bit integer data
    Byte(u8),
    /// Unsigned 8bit integer that gets mapped to a FieldType enum
    Enum(u8),
    /// Signed 8bit integer data
    SInt8(i8),
    /// Unsigned 8bit integer data
    UInt8(u8),
    /// Signed 16bit integer data
    SInt16(i16),
    /// Unsigned 16bit integer data
    UInt16(u16),
    /// Signed 32bit integer data
    SInt32(i32),
    /// Unsigned 32bit integer data
    UInt32(u32),
    /// UTF-8 format string data
    String(String),
    /// 32bit floating point data
    Float32(f32),
    /// 64bit floating point data
    Float64(f64),
    /// Unsigned 8bit integer data where the invalid value is `0x0` instead of `0xFF`
    UInt8z(u8),
    /// Unsigned 16bit integer data where the invalid value is `0x0` instead of `0xFFFF`
    UInt16z(u16),
    /// Unsigned 16bit integer data where the invalid value is `0x0` instead of `0xFFFFFFFF`
    UInt32z(u32),
    /// Signed 64bit integer data
    SInt64(i64),
    /// Unsigned 64bit integer data
    UInt64(u64),
    /// Unsigned 64bit integer data where the invalid value is `0x0` instead of `0xFFFFFFFFFFFFFFFF`
    UInt64z(u64),
    /// Array of Values, while this allows nested arrays and mixed types this is not possible
    /// in a properly formatted FIT file
    Array(Vec<Self>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Value::Timestamp(val) => write!(f, "{}", val),
            Value::Byte(val) => write!(f, "{}", val),
            Value::Enum(val) => write!(f, "{}", val),
            Value::SInt8(val) => write!(f, "{}", val),
            Value::UInt8(val) => write!(f, "{}", val),
            Value::UInt8z(val) => write!(f, "{}", val),
            Value::SInt16(val) => write!(f, "{}", val),
            Value::UInt16(val) => write!(f, "{}", val),
            Value::UInt16z(val) => write!(f, "{}", val),
            Value::SInt32(val) => write!(f, "{}", val),
            Value::UInt32(val) => write!(f, "{}", val),
            Value::UInt32z(val) => write!(f, "{}", val),
            Value::SInt64(val) => write!(f, "{}", val),
            Value::UInt64(val) => write!(f, "{}", val),
            Value::UInt64z(val) => write!(f, "{}", val),
            Value::Float32(val) => write!(f, "{}", val),
            Value::Float64(val) => write!(f, "{}", val),
            Value::String(val) => write!(f, "{}", val),
            Value::Array(vals) => write!(f, "{:?}", vals), // printing arrays is hard
        }
    }
}

impl convert::TryInto<f64> for Value {
    type Error = error::Error;

    fn try_into(self) -> Result<f64> {
        match self {
            Value::Timestamp(val) => Ok(val.timestamp() as f64),
            Value::Byte(val) => Ok(val as f64),
            Value::Enum(val) => Ok(val as f64),
            Value::SInt8(val) => Ok(val as f64),
            Value::UInt8(val) => Ok(val as f64),
            Value::UInt8z(val) => Ok(val as f64),
            Value::SInt16(val) => Ok(val as f64),
            Value::UInt16(val) => Ok(val as f64),
            Value::UInt16z(val) => Ok(val as f64),
            Value::SInt32(val) => Ok(val as f64),
            Value::UInt32(val) => Ok(val as f64),
            Value::UInt32z(val) => Ok(val as f64),
            Value::SInt64(val) => Ok(val as f64),
            Value::UInt64(val) => Ok(val as f64),
            Value::UInt64z(val) => Ok(val as f64),
            Value::Float32(val) => Ok(val as f64),
            Value::Float64(val) => Ok(val),
            Value::String(_) => {
                Err(ErrorKind::ValueError(format!("cannot convert {} into an f64", self)).into())
            }
            Value::Array(_) => {
                Err(ErrorKind::ValueError(format!("cannot convert {} into an f64", self)).into())
            }
        }
    }
}

impl convert::TryInto<i64> for Value {
    type Error = error::Error;

    fn try_into(self) -> Result<i64> {
        match self {
            Value::Timestamp(val) => Ok(val.timestamp()),
            Value::Byte(val) => Ok(val as i64),
            Value::Enum(val) => Ok(val as i64),
            Value::SInt8(val) => Ok(val as i64),
            Value::UInt8(val) => Ok(val as i64),
            Value::UInt8z(val) => Ok(val as i64),
            Value::SInt16(val) => Ok(val as i64),
            Value::UInt16(val) => Ok(val as i64),
            Value::UInt16z(val) => Ok(val as i64),
            Value::SInt32(val) => Ok(val as i64),
            Value::UInt32(val) => Ok(val as i64),
            Value::UInt32z(val) => Ok(val as i64),
            Value::SInt64(val) => Ok(val),
            Value::UInt64(val) => Ok(val as i64),
            Value::UInt64z(val) => Ok(val as i64),
            Value::Float32(_) => {
                Err(ErrorKind::ValueError(format!("cannot convert {} into an i64", self)).into())
            }
            Value::Float64(_) => {
                Err(ErrorKind::ValueError(format!("cannot convert {} into an i64", self)).into())
            }
            Value::String(_) => {
                Err(ErrorKind::ValueError(format!("cannot convert {} into an i64", self)).into())
            }
            Value::Array(_) => {
                Err(ErrorKind::ValueError(format!("cannot convert {} into an i64", self)).into())
            }
        }
    }
}

/// Describes a field value along with its defined units (if any), this struct is useful for
/// serializing data in a key-value store where the key is either the name or definition number
/// since it can be created from a `FitDataField` with minimal data cloning.
#[derive(Clone, Debug, Serialize)]
pub struct ValueWithUnits {
    value: Value,
    units: String,
}

impl ValueWithUnits {
    /// Create a new value with the given information
    pub fn new(value: Value, units: String) -> Self {
        ValueWithUnits { value, units }
    }
}

impl convert::From<FitDataField> for ValueWithUnits {
    fn from(field: FitDataField) -> Self {
        ValueWithUnits::new(field.value, field.units)
    }
}

impl fmt::Display for ValueWithUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.units.is_empty() {
            write!(f, "{}", self.value)
        } else {
            write!(f, "{} {}", self.value, self.units)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_activity() {
        let data = include_bytes!("../tests/fixtures/Activity.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 22);
    }

    #[test]
    fn parse_developer_data() {
        let data = include_bytes!("../tests/fixtures/DeveloperData.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 6);
    }

    #[test]
    fn parse_monitoring_file() {
        let data = include_bytes!("../tests/fixtures/MonitoringFile.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 355);
    }

    #[test]
    fn parse_settings() {
        let data = include_bytes!("../tests/fixtures/Settings.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 3);
    }

    #[test]
    fn parse_weight_scale_multi_user() {
        let data = include_bytes!("../tests/fixtures/WeightScaleMultiUser.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 7);
    }

    #[test]
    fn parse_weight_scale_single_user() {
        let data = include_bytes!("../tests/fixtures/WeightScaleSingleUser.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 6);
    }

    #[test]
    fn parse_workout_custom_target_values() {
        let data = include_bytes!("../tests/fixtures/WorkoutCustomTargetValues.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 6);
    }

    #[test]
    fn parse_workout_individual_steps() {
        let data = include_bytes!("../tests/fixtures/WorkoutIndividualSteps.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 6);
    }

    #[test]
    fn parse_workout_repeat_greater_than_step() {
        let data = include_bytes!("../tests/fixtures/WorkoutRepeatGreaterThanStep.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 7);
    }

    #[test]
    fn parse_workout_repeat_steps() {
        let data = include_bytes!("../tests/fixtures/WorkoutRepeatSteps.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 7);
    }

    #[test]
    fn parse_garmin_fenix_5_bike() {
        // this test case includes a FIT file with a string field, which was broken in v0.1.0
        // and fixed in v0.1.1
        let data = include_bytes!("../tests/fixtures/garmin-fenix-5-bike.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 143);
    }

    #[test]
    fn parse_sample_mulitple_header() {
        // this test case includes a chained FIT file
        let data = include_bytes!("../tests/fixtures/sample_mulitple_header.fit").to_vec();
        let fit_data = from_bytes(&data).unwrap();
        assert_eq!(fit_data.len(), 3023);
    }
}
