//! Helper functions and structures needed to decode a FIT file using the defined profile.
use super::parser::FitDataMessage;
use crate::error::{ErrorKind, Result};
use crate::profile::{
    get_field_variant_as_string, ComponentFieldInfo, FieldDataType, FieldInfo, MesgNum, MessageInfo,
};
use crate::{FitDataField, FitDataRecord, Value};
use chrono::{DateTime, Duration, Local, NaiveDate, TimeZone};
use std::collections::HashMap;
use std::convert::{From, TryInto};
use std::f64::EPSILON;
use std::iter::IntoIterator;

impl Value {
    /// Convert the value into a vector of bytes
    fn to_ne_bytes(&self) -> Vec<u8> {
        match self {
            Value::Byte(val) => vec![*val as u8],
            Value::Enum(val) => vec![*val as u8],
            Value::SInt8(val) => vec![*val as u8],
            Value::UInt8(val) => vec![*val as u8],
            Value::SInt16(val) => val.to_ne_bytes().to_vec(),
            Value::UInt16(val) => val.to_ne_bytes().to_vec(),
            Value::SInt32(val) => val.to_ne_bytes().to_vec(),
            Value::UInt32(val) => val.to_ne_bytes().to_vec(),
            Value::String(val) => val.as_bytes().to_vec(),
            Value::Timestamp(val) => val.timestamp().to_ne_bytes().to_vec(),
            Value::Float32(val) => val.to_ne_bytes().to_vec(),
            Value::Float64(val) => val.to_ne_bytes().to_vec(),
            Value::UInt8z(val) => vec![*val as u8],
            Value::UInt16z(val) => val.to_ne_bytes().to_vec(),
            Value::UInt32z(val) => val.to_ne_bytes().to_vec(),
            Value::SInt64(val) => val.to_ne_bytes().to_vec(),
            Value::UInt64(val) => val.to_ne_bytes().to_vec(),
            Value::UInt64z(val) => val.to_ne_bytes().to_vec(),
            Value::Array(vals) => vals.iter().flat_map(|v| v.to_ne_bytes()).collect(),
        }
    }
}

/// Stores the timestamp offset from the FIT reference date in seconds
#[derive(Debug, Copy, Clone)]
enum TimestampField {
    Local(i64),
    Utc(i64),
}

impl TimestampField {
    /// Return the timestamp as an i64
    fn as_i64(&self) -> i64 {
        match self {
            Self::Local(value) => *value,
            Self::Utc(value) => *value,
        }
    }

    /// converts offset value into a proper timestamp
    fn to_date_time(self) -> DateTime<Local> {
        // reference date defined in FIT profile, it's either in UTC or local TZ
        let ref_date = NaiveDate::from_ymd(1989, 12, 31).and_hms(0, 0, 0);
        match self {
            Self::Local(value) => {
                TimeZone::from_local_datetime(&Local, &ref_date).unwrap() + Duration::seconds(value)
            }
            Self::Utc(value) => {
                TimeZone::from_utc_datetime(&Local, &ref_date) + Duration::seconds(value)
            }
        }
    }
}

impl From<TimestampField> for Value {
    fn from(timestamp: TimestampField) -> Value {
        Value::Timestamp(timestamp.to_date_time())
    }
}

/// Decodes a raw FitDataMessage using the defined profile. Additional logic is used to handle
/// values that need to accumlate across multiple messages as well as applying the
/// time offset to the current base timestamp.
pub struct Decoder {
    base_timestamp: TimestampField,
    accumulate_fields: HashMap<u32, Value>,
}

impl Decoder {
    /// Create a new decoder
    pub fn new() -> Self {
        Decoder {
            base_timestamp: TimestampField::Utc(0),
            accumulate_fields: HashMap::new(),
        }
    }

    /// Reset accumation related fields
    pub fn reset(&mut self) {
        self.base_timestamp = TimestampField::Utc(0);
        self.accumulate_fields = HashMap::new();
    }

    /// Decode a raw FIT data message by applying the defined profile
    pub fn decode_message(&mut self, message: FitDataMessage) -> Result<FitDataRecord> {
        let mesg_num = MesgNum::from(message.global_message_number());
        let mesg_info = mesg_num.message_info();
        let mut record = FitDataRecord::new(mesg_num);

        // check if we have a real timestamp field to set the reference
        if let Some(Some(value)) = message.fields().get(&253) {
            let value = value.clone();
            self.base_timestamp =
                if let Some(info) = get_message_field(&mesg_info, 253, &HashMap::new()) {
                    if let FieldDataType::LocalDateTime = info.field_type() {
                        TimestampField::Local(value.try_into().unwrap_or(0))
                    } else {
                        TimestampField::Utc(value.try_into().unwrap_or(0))
                    }
                } else {
                    // default to assuming we have a UTC timestamp
                    TimestampField::Utc(value.try_into().unwrap_or(0))
                }
        }

        // process raw data
        for field in self.build_data_fields_from_map(mesg_info, message.fields())? {
            record.push(field);
        }

        // Add a timestamp field if we have a time offset
        if let Some(time_offset) = message.time_offset() {
            record.push(FitDataField::new(
                String::from("timestamp"),
                253,
                self.update_timestamp(time_offset),
                String::new(),
            ));
        }

        // TODO: process developer fields

        Ok(record)
    }

    /// Build processed field values from the raw data mapping
    fn build_data_fields_from_map(
        &mut self,
        mesg_info: MessageInfo,
        data_map: &HashMap<u8, Option<Value>>,
    ) -> Result<Vec<FitDataField>> {
        // initialize process queue with field info for parsed, valid fields.
        let msg_num = mesg_info.global_message_number().as_u16();
        let mut data_fields = Vec::new();
        let mut data_map: HashMap<u8, Value> = data_map
            .iter()
            .filter_map(|(key, val)| val.as_ref().map(|v| (*key, v.clone())))
            .collect();

        let mut process_queue: Vec<(u8, Option<FieldInfo>)> = data_map
            .keys()
            .map(|k| (*k, get_message_field(&mesg_info, *k, &data_map).cloned()))
            .collect();

        while !process_queue.is_empty() {
            let (def_num, field_info) = process_queue.remove(0);
            let mut value = data_map[&def_num].clone();

            if let Some(field_info) = field_info {
                // check for components, the decomposition is profile specific so
                // we dont store the parent field because we want the JSON to be
                // profile agnostic
                if field_info.components().is_empty() {
                    if field_info.accumulate() {
                        value = self.accumlate_value(msg_num, def_num, value)?;
                    }
                    data_fields.push(data_field_with_info(&field_info, value)?);
                } else {
                    let (infos, values): (Vec<_>, Vec<_>) =
                        expand_components(&field_info, &value).into_iter().unzip();
                    // add all data to map first and then update process queue since reference fields
                    // are data dependent
                    for (comp_info, comp_value) in infos.iter().zip(values.into_iter()) {
                        data_map.insert(comp_info.dest_def_number(), comp_value);
                    }
                    for comp_info in infos {
                        let dest_def_number = comp_info.dest_def_number();
                        let old_field_info =
                            get_message_field(&mesg_info, comp_info.dest_def_number(), &data_map);
                        let new_field_info = old_field_info.map(|i| comp_info.to_field_info(i));
                        process_queue.push((dest_def_number, new_field_info));
                    }
                }
            } else {
                data_fields.push(unknown_field(def_num, value));
            }
        }

        data_fields.sort_by_key(|f| f.number());
        Ok(data_fields)
    }

    /// Increment the stored field value
    fn accumlate_value(&mut self, msg_num: u16, def_num: u8, value: Value) -> Result<Value> {
        // use macro to duplicate same type only addition logic
        macro_rules! only_add_like_values {
            ($key:ident, $val:ident, $stored_value:ident, $variant:ident) => {
                if let Value::$variant(other) = value {
                    let value = Value::$variant($val + other);
                    self.accumulate_fields.insert($key, value.clone());
                    Ok(value)
                } else {
                    Err(ErrorKind::ValueError(format!(
                        "Mixed type addition {} and {} cannot be combined",
                        $stored_value, value
                    ))
                    .into())
                }
            };
        }

        let key = (msg_num as u32) << 8 | def_num as u32;
        if let Some(stored_value) = self.accumulate_fields.get(&key) {
            match stored_value {
                Value::Timestamp(_) => Err(ErrorKind::ValueError(
                    "Cannot accumlate timestamp fields".to_string(),
                )
                .into()),
                Value::Byte(val) => only_add_like_values!(key, val, stored_value, Byte),
                Value::Enum(_) => {
                    Err(ErrorKind::ValueError("Cannot accumlate enum fields".to_string()).into())
                }
                Value::SInt8(val) => only_add_like_values!(key, val, stored_value, SInt8),
                Value::UInt8(val) => only_add_like_values!(key, val, stored_value, UInt8),
                Value::UInt8z(val) => only_add_like_values!(key, val, stored_value, UInt8z),
                Value::SInt16(val) => only_add_like_values!(key, val, stored_value, SInt16),
                Value::UInt16(val) => only_add_like_values!(key, val, stored_value, UInt16),
                Value::UInt16z(val) => only_add_like_values!(key, val, stored_value, UInt16z),
                Value::SInt32(val) => only_add_like_values!(key, val, stored_value, SInt32),
                Value::UInt32(val) => only_add_like_values!(key, val, stored_value, UInt32),
                Value::UInt32z(val) => only_add_like_values!(key, val, stored_value, UInt32z),
                Value::SInt64(val) => only_add_like_values!(key, val, stored_value, SInt64),
                Value::UInt64(val) => only_add_like_values!(key, val, stored_value, UInt64),
                Value::UInt64z(val) => only_add_like_values!(key, val, stored_value, UInt64z),
                Value::Float32(val) => only_add_like_values!(key, val, stored_value, Float32),
                Value::Float64(val) => only_add_like_values!(key, val, stored_value, Float64),
                Value::String(_) => {
                    Err(ErrorKind::ValueError("Cannot accumlate string fields".to_string()).into())
                }
                Value::Array(_) => {
                    Err(ErrorKind::ValueError("Cannot accumlate array fields".to_string()).into())
                }
            }
        } else {
            self.accumulate_fields.insert(key, value.clone());
            Ok(value)
        }
    }

    /// Update the timestamp with a new offset and return the value
    fn update_timestamp(&mut self, offset: u8) -> Value {
        let offset: i64 = offset as i64;
        let mask: i64 = 31; // last 5 significant bits of value
        let mut value = offset + (self.base_timestamp.as_i64() & !mask);
        // account for rollover if needed
        if offset < (self.base_timestamp.as_i64() & mask) {
            value += 32;
        }

        // update stored value and return
        match self.base_timestamp {
            TimestampField::Local(_) => {
                self.base_timestamp = TimestampField::Local(value);
            }
            TimestampField::Utc(_) => {
                self.base_timestamp = TimestampField::Utc(value);
            }
        }

        Value::from(self.base_timestamp)
    }
}

/// Fetch the information for a specific field, if the field contains subfields then
/// we use the data values provided to try and de-reference it and return the subfield
/// info instead
fn get_message_field<'a>(
    msg: &'a MessageInfo,
    key: u8,
    data_map: &HashMap<u8, Value>,
) -> Option<&'a FieldInfo> {
    if let Some(field) = msg.fields().get(&key) {
        // check against subfields
        for (num, val, sub_info) in field.subfields() {
            if let Some(v) = data_map.get(num) {
                if v.clone().try_into().map_or(false, |v: i64| v == *val) {
                    return Some(sub_info);
                }
            }
        }
        // fallback to initial field info as default return
        return Some(field);
    }
    None
}

/// Applies a bitmask to the value and uses the field info to derive additional fields based on the
/// defined components
fn expand_components<'a>(
    field_info: &'a FieldInfo,
    value: &Value,
) -> Vec<(&'a ComponentFieldInfo, Value)> {
    // extract out each field by masking specific bits, spanning 1 or more bytes
    let bit_mask = [1u8, 2u8, 4u8, 8u8, 16u8, 32u8, 64u8, 128u8];
    let mut bytes = value.to_ne_bytes().into_iter();
    let mut components = Vec::new();
    let mut byte = bytes.next().unwrap_or(0);
    let mut bit_pos = 0;
    for comp_fld in field_info.components() {
        let mut tmp: u64 = 0;
        for pos in 0..comp_fld.bits() {
            tmp |= (((byte & bit_mask[bit_pos]) >> bit_pos) as u64) << pos;
            if bit_pos == 7 {
                byte = bytes.next().unwrap_or(0);
                bit_pos = 0;
            } else {
                bit_pos += 1;
            }
        }
        components.push((comp_fld, Value::UInt64(tmp)));
    }

    components
}

/// Applies any necessary value conversions based on the field specification
fn convert_value(field_info: &FieldInfo, value: Value) -> Result<Value> {
    // for array types return inner vector unmodified
    if let Value::Array(vals) = value {
        return Ok(Value::Array(vals));
    }

    // handle time types specially, if for some reason I can't convert to an integer we will
    // just dump the reference timestamp by passing it a 0
    match field_info.field_type() {
        FieldDataType::DateTime => {
            return Ok(Value::from(TimestampField::Utc(
                value.try_into().unwrap_or(0),
            )));
        }
        FieldDataType::LocalDateTime => {
            return Ok(Value::from(TimestampField::Local(
                value.try_into().unwrap_or(0),
            )));
        }
        _ => (),
    }

    // convert enum or rescale integer value into floating point
    if field_info.field_type().is_enum_type() {
        let val: i64 = value.try_into()?;
        Ok(Value::String(get_field_variant_as_string(
            field_info.field_type(),
            val,
        )))
    } else if ((field_info.scale() - 1.0).abs() > EPSILON)
        || ((field_info.offset() - 0.0).abs() > EPSILON)
    {
        let val: f64 = value.try_into()?;
        Ok(Value::Float64(
            val / field_info.scale() - field_info.offset(),
        ))
    } else {
        Ok(value)
    }
}

/// Build a data field using the provided FIT profile information
fn data_field_with_info(field_info: &FieldInfo, value: Value) -> Result<FitDataField> {
    let value = convert_value(field_info, value)?;
    Ok(FitDataField::new(
        field_info.name().to_string(),
        field_info.def_number(),
        value,
        field_info.units().to_string(),
    ))
}

/// Create an "unknown" field as a placeholder if we don't have any field information
fn unknown_field(field_def_num: u8, value: Value) -> FitDataField {
    FitDataField::new(
        format!("unknown_field_{}", field_def_num),
        field_def_num,
        value,
        String::new(),
    )
}
