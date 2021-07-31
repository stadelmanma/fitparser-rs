use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use std::collections::{BTreeMap, HashSet};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

// the fields in these structs are mostly duplicated from code in src/profile/parser.rs
#[derive(Clone, Debug)]
pub struct FitProfile {
    version: String,
    field_types: Vec<FieldTypeDefintion>,
    messages: Vec<MessageDefinition>,
}

#[derive(Clone, Debug)]
struct FieldTypeDefintion {
    name: String,
    titlized_name: String,
    base_type: &'static str,
    comment: Option<String>,
    variant_map: BTreeMap<i64, FieldTypeVariant>,
}

impl FieldTypeDefintion {
    fn new(name: String, base_type: &'static str, comment: Option<String>) -> Self {
        FieldTypeDefintion {
            name: name.clone(),
            titlized_name: titlecase_string(&name),
            base_type,
            comment,
            variant_map: BTreeMap::new(),
        }
    }

    /// Generate an enum from the field type variants
    fn generate_enum(&self, out: &mut File) -> Result<(), std::io::Error> {
        if let Some(v) = &self.comment {
            writeln!(out, "/// {}", v)?;
        }
        writeln!(
            out,
            "#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]"
        )?;
        writeln!(out, "pub enum {} {{", titlecase_string(&self.name))?;
        for variant in self.variant_map.values() {
            variant.write_variant_line(out)?;
        }
        writeln!(out, "UnknownVariant({}),", self.base_type)?;
        writeln!(out, "}}")?;

        self.generate_impl(out)
    }

    /// generate to/from integer and to_string implementation for field type enum
    fn generate_impl(&self, out: &mut File) -> Result<(), std::io::Error> {
        writeln!(out, "impl {} {{", titlecase_string(&self.name))?;

        writeln!(out, "pub fn as_{0}(self) -> {0} {{", self.base_type)?;
        writeln!(out, "match self {{")?;
        for variant in self.variant_map.values() {
            writeln!(
                out,
                "{}::{} => {},",
                titlecase_string(&self.name),
                variant.titlized_name(),
                variant.value
            )?;
        }
        writeln!(
            out,
            "{}::UnknownVariant(value) => value",
            titlecase_string(&self.name)
        )?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;

        writeln!(out, "pub fn as_i64(self) -> i64 {{")?;
        writeln!(out, "self.as_{}() as i64", self.base_type)?;
        writeln!(out, "}}")?;

        writeln!(out, "}}")?;

        writeln!(
            out,
            "impl fmt::Display for {} {{",
            titlecase_string(&self.name)
        )?;
        writeln!(
            out,
            "fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{"
        )?;
        writeln!(out, "match &self {{")?;
        for variant in self.variant_map.values() {
            writeln!(
                out,
                "{}::{} => write!(f, \"{}\"),",
                titlecase_string(&self.name),
                variant.titlized_name(),
                variant.name
            )?;
        }
        writeln!(
            out,
            "{}::UnknownVariant(value) => write!(f, \"unknown_variant_{{}}\", *value)",
            titlecase_string(&self.name)
        )?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;

        writeln!(
            out,
            "impl convert::From<{}> for {} {{",
            self.base_type,
            titlecase_string(&self.name)
        )?;
        writeln!(out, "fn from(value: {0}) -> Self {{", self.base_type)?;
        writeln!(out, "match value {{")?;
        for variant in self.variant_map.values() {
            writeln!(
                out,
                "{} => {}::{},",
                variant.value,
                titlecase_string(&self.name),
                variant.titlized_name()
            )?;
        }
        writeln!(
            out,
            " _ => {}::UnknownVariant(value)",
            titlecase_string(&self.name)
        )?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;

        writeln!(
            out,
            "impl convert::From<i64> for {} {{",
            titlecase_string(&self.name)
        )?;
        writeln!(out, "fn from(value: i64) -> Self {{")?;
        writeln!(
            out,
            "{0}::from(value as {1})",
            titlecase_string(&self.name),
            self.base_type
        )?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;

        writeln!(
            out,
            "impl Serialize for {} {{",
            titlecase_string(&self.name)
        )?;
        writeln!(
            out,
            "fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>"
        )?;
        writeln!(out, "where")?;
        writeln!(out, "S: Serializer,")?;
        writeln!(out, "{{")?;
        writeln!(out, "serializer.serialize_str(&self.to_string())")?;
        writeln!(out, "}}")?;
        writeln!(out, "}}")?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct FieldTypeVariant {
    name: String,
    value: i64,
    comment: Option<String>,
}

impl FieldTypeVariant {
    fn titlized_name(&self) -> String {
        let mut titlized_name = titlecase_string(&self.name);

        // First letter isn't between A-Z in ASCII
        let first_let = titlized_name.as_bytes()[0];
        if !(65..=90).contains(&first_let) {
            titlized_name = format!("Name{}", titlized_name);
        }

        titlized_name
    }

    fn write_variant_line(&self, out: &mut File) -> Result<(), std::io::Error> {
        if let Some(v) = &self.comment {
            writeln!(out, "/// {}\n{},", v, self.titlized_name())
        } else {
            writeln!(out, "{},", self.titlized_name())
        }
    }
}

#[derive(Clone, Debug)]
struct MessageDefinition {
    name: String,
    comment: Option<String>,
    field_map: BTreeMap<u8, MessageFieldDefinition>,
}

impl MessageDefinition {
    fn new(name: &str, comment: Option<String>) -> Self {
        MessageDefinition {
            name: name.to_string(),
            comment,
            field_map: BTreeMap::new(),
        }
    }

    fn get_field_by_name(&self, name: &str) -> &MessageFieldDefinition {
        for field in self.field_map.values() {
            if field.name == name {
                return field;
            }
        }
        panic!("No field with name: {:?}", name);
    }

    fn function_name(&self) -> String {
        format!("{}_message", self.name)
    }

    fn write_function_def(&self, out: &mut File) -> Result<(), std::io::Error> {
        if let Some(v) = &self.comment {
            writeln!(out, "/// {}", v)?;
        }
        writeln!(out, "pub fn {}() -> MessageInfo {{", self.function_name())?;
        writeln!(out, "    let mut fields = HashMap::new();")?;
        for field in self.field_map.values() {
            field.generate_field_info_struct(out, self, "field")?;
            writeln!(out, "fields.insert({}, field);", field.def_number)?;
        }
        writeln!(out, "    MessageInfo {{")?;
        writeln!(out, "        name: \"{}\",", self.name)?;
        writeln!(
            out,
            "        global_message_number: MesgNum::{},",
            titlecase_string(&self.name)
        )?;
        writeln!(out, "        fields")?;
        writeln!(out, "    }}")?;
        writeln!(out, "}}")?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct MessageFieldDefinition {
    def_number: u8,
    name: String,
    field_type: String,
    scale: f64,
    offset: f64,
    units: String,
    accumulate: bool,
    subfields: Vec<(String, String, MessageFieldDefinition)>,
    components: Vec<MessageFieldComponent>,
    comment: Option<String>,
}

impl MessageFieldDefinition {
    pub fn generate_field_info_struct(
        &self,
        out: &mut File,
        mesg: &MessageDefinition,
        var_name: &str,
    ) -> Result<(), std::io::Error> {
        let subfield_var: &'static str;
        if self.subfields.is_empty() {
            subfield_var = "Vec::new()";
        } else {
            subfield_var = "subfields";
            writeln!(out, "let mut {} = Vec::new();", subfield_var)?;
            for (fld_name, fld_value, sub_info) in &self.subfields {
                sub_info.generate_field_info_struct(out, mesg, "sub_fld")?;
                let ref_field = mesg.get_field_by_name(fld_name);
                writeln!(
                    out,
                    "subfields.push(({}, {}::{}.as_i64(), sub_fld));",
                    ref_field.def_number,
                    ref_field.field_type,
                    titlecase_string(fld_value),
                )?;
            }
        }

        let components_var: &'static str;
        if self.components.is_empty() {
            components_var = "Vec::new()";
        } else {
            components_var = "components";
            writeln!(out, "let mut {} = Vec::new();", components_var)?;
            for comp_info in &self.components {
                comp_info.generate_comp_field_info_struct(out, mesg, "comp_fld")?;
                writeln!(out, "components.push(comp_fld);")?;
            }
        }

        if let Some(v) = &self.comment {
            writeln!(out, "// {}", v)?;
        }
        writeln!(
            out,
            "    let {} = FieldInfo {{
            name: \"{}\",
            field_type: FieldDataType::{},
            def_number: {},
            scale: {:.6},
            offset: {:.6},
            units: \"{}\",
            accumulate: {},
            subfields: {},
            components: {},
        }};",
            var_name,
            self.name,
            self.field_type,
            self.def_number,
            self.scale,
            self.offset,
            self.units,
            self.accumulate,
            subfield_var,
            components_var
        )?;

        Ok(())
    }
}

#[derive(Clone, Debug)]
struct MessageFieldComponent {
    name: String,
    scale: f64,
    offset: f64,
    units: String,
    bits: u8,
    accumulate: bool,
}

impl MessageFieldComponent {
    fn generate_comp_field_info_struct(
        &self,
        out: &mut File,
        mesg: &MessageDefinition,
        var_name: &str,
    ) -> Result<(), std::io::Error> {
        let dest_def_number = mesg.get_field_by_name(&self.name).def_number;
        writeln!(
            out,
            "let {} = ComponentFieldInfo {{
            dest_def_number: {},
            scale: {:.6},
            offset: {:.6},
            units: \"{}\",
            bits: {},
            accumulate: {},
        }};",
            var_name,
            dest_def_number,
            self.scale,
            self.offset,
            self.units,
            self.bits,
            self.accumulate,
        )?;

        Ok(())
    }
}

/// Convert an ASCII string to title/camel case
fn titlecase_string(value: &str) -> String {
    let mut words: Vec<String> = value.split('_').map(|v| v.to_string()).collect();

    for word in &mut words {
        if let Some(l) = word.get_mut(0..1) {
            l.make_ascii_uppercase();
        }
    }

    words.join("")
}

macro_rules! split_csv_string ( ($value:expr) => ( {$value.split(',').map(|v| v.trim().to_string())} ););

/// Match a base type string to a rust type for enum generation
fn base_type_to_rust_type(base_type_str: &str) -> &'static str {
    match base_type_str {
        "enum" => "u8",
        "sint8" => "i8",
        "uint8" => "u8",
        "uint8z" => "u8",
        "sint16" => "i16",
        "uint16" => "u16",
        "uint16z" => "u16",
        "sint32" => "i32",
        "uint32" => "u32",
        "uint32z" => "u32",
        _ => panic!("unsupported base_type for enum field: {}", base_type_str),
    }
}

/// match the field type string to a simple type or an enum
fn field_type_str_to_field_type(field_type_str: &str) -> String {
    match field_type_str {
        "sint8" => "SInt8".to_string(),
        "uint8" => "UInt8".to_string(),
        "sint16" => "SInt16".to_string(),
        "uint16" => "UInt16".to_string(),
        "sint32" => "SInt32".to_string(),
        "uint32" => "UInt32".to_string(),
        "string" => "String".to_string(),
        "float32" => "Float32".to_string(),
        "float64" => "Float64".to_string(),
        "uint8z" => "UInt8z".to_string(),
        "uint16z" => "UInt16z".to_string(),
        "uint32z" => "UInt32z".to_string(),
        "byte" => "Byte".to_string(),
        "sint64" => "SInt64".to_string(),
        "uint64" => "UInt64".to_string(),
        "uint64z" => "UInt64z".to_string(),
        _ => titlecase_string(field_type_str),
    }
}

fn generate_main_field_type_enum(
    field_types: &[FieldTypeDefintion],
    out: &mut File,
) -> Result<(), std::io::Error> {
    let base_types = vec![
        "Bool", "SInt8", "UInt8", "SInt16", "UInt16", "SInt32", "UInt32", "String", "Float32",
        "Float64", "UInt8z", "UInt16z", "UInt32z", "Byte", "SInt64", "UInt64", "UInt64z",
    ];
    let mut is_enum_force_false = HashSet::new();
    is_enum_force_false.insert("date_time".to_string());
    is_enum_force_false.insert("local_date_time".to_string());

    writeln!(
        out,
        "
/// Describe all possible data types of a field
///
/// The Enum type's value is actually an enum of enums.
#[derive(Clone, Copy, Debug)]
pub enum FieldDataType {{
"
    )?;
    for type_name in base_types {
        writeln!(out, " {},", type_name)?;
    }
    for field_type in field_types {
        writeln!(out, "{},", field_type.titlized_name)?;
    }

    writeln!(out, "}}")?;

    writeln!(out, "impl FieldDataType {{")?;
    writeln!(out, "    pub fn is_enum_type(self) -> bool {{")?;
    writeln!(out, "        match self {{")?;
    for field_type in field_types {
        if !field_type.variant_map.is_empty() && !is_enum_force_false.contains(&field_type.name) {
            writeln!(out, "FieldDataType::{} => true,", field_type.titlized_name)?;
        }
    }
    writeln!(out, "            _ => false,")?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;

    writeln!(
        out,
        "pub fn get_field_variant_as_string(field_type: FieldDataType , value: i64) -> String {{"
    )?;
    writeln!(out, "    match field_type {{")?;
    for field_type in field_types {
        if !field_type.variant_map.is_empty() && !is_enum_force_false.contains(&field_type.name) {
            writeln!(
                out,
                "        FieldDataType::{0} => {0}::from(value).to_string(),",
                field_type.titlized_name
            )?;
        }
    }
    writeln!(out, "        _ => format!(\"Undefined{{}}\", value),")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;

    Ok(())
}

fn process_types(sheet: Range<DataType>) -> Vec<FieldTypeDefintion> {
    let mut field_types: Vec<FieldTypeDefintion> = Vec::new();

    for row in sheet.rows().skip(1) {
        if !row[0].is_empty() {
            // extract enum name
            let enum_name = match row[0].get_string() {
                Some(v) => v.to_string(),
                None => panic!("Enum type name must be a string row={:?}.", row),
            };

            // extract base type and convert to its rust equivalent
            let rust_type = match row[1].get_string() {
                Some(v) => base_type_to_rust_type(v),
                None => panic!("Base type name must be a string row={:?}.", row),
            };
            let comment = row[4].get_string().map(|v| v.to_string());
            field_types.push(FieldTypeDefintion::new(enum_name, rust_type, comment));
        } else if !row[2].is_empty() {
            let field_type = match field_types.last_mut() {
                Some(v) => v,
                None => panic!("field_types vector was empty!"),
            };
            // add enum variant
            // extract enum name
            let name = match row[2].get_string() {
                Some(v) => v.to_string(),
                None => panic!("Enum variant name must be a string row={:?}.", row),
            };

            // handle mix of numeric and hex string data values
            let value = match &row[3] {
                DataType::Float(v) => *v as i64,
                DataType::Int(v) => *v,
                DataType::String(v) => i64::from_str_radix(&v[2..], 16).unwrap(),
                _ => {
                    panic!("Unsupported enum variant value data type row={:?}.", row);
                }
            };
            let comment = row[4].get_string().map(|v| v.to_string());
            field_type.variant_map.insert(
                value,
                FieldTypeVariant {
                    name,
                    value,
                    comment,
                },
            );
        }
    }

    field_types
}

fn parse_message_field_components(row: &[DataType]) -> Vec<MessageFieldComponent> {
    let mut components = Vec::new();

    // parse out the fields into iterators
    let names: Vec<String> = match row[5].get_string() {
        Some(v) => split_csv_string!(v).collect(),
        None => {
            return components;
        }
    };
    let cols: Vec<String> = row[6..=10].iter().map(|v| v.to_string()).collect();
    let mut scales = split_csv_string!(cols[0]).map(|s| s.parse::<f64>().ok());
    let mut offsets = split_csv_string!(cols[1]).map(|s| s.parse::<f64>().ok());
    let mut units = split_csv_string!(cols[2]);
    let mut bits = split_csv_string!(cols[3]).map(|s| s.parse::<u8>().ok());
    let mut accumulate = split_csv_string!(cols[4]).map(|s| s == "1");

    // build each component
    for name in names {
        components.push(MessageFieldComponent {
            name,
            scale: scales.next().flatten().unwrap_or(1.0),
            offset: offsets.next().flatten().unwrap_or(0.0),
            units: units.next().unwrap_or_default(),
            bits: bits
                .next()
                .flatten()
                .unwrap_or_else(|| panic!("Could not parse bits value for row: {:?}", row)),
            accumulate: accumulate.next().unwrap_or(false),
        });
    }
    components
}

fn new_message_field_definition(row: &[DataType]) -> MessageFieldDefinition {
    let def_number = match row[1] {
        DataType::Float(v) => v as u8,
        DataType::Int(v) => v as u8,
        _ => panic!("Field defintiton number must be an integer, row={:?}.", row),
    };
    let name = row[2]
        .get_string()
        .unwrap_or_else(|| panic!("Field name must be a string, row={:?}.", row));
    let ftype = row[3]
        .get_string()
        .unwrap_or_else(|| panic!("Field type must be a string, row={:?}.", row));
    let components = parse_message_field_components(row);
    let comment = row[13].get_string().map(|v| v.to_string());

    MessageFieldDefinition {
        def_number,
        name: name.to_string(),
        field_type: field_type_str_to_field_type(ftype),
        scale: row[6].get_float().unwrap_or(1.0),
        offset: row[7].get_float().unwrap_or(0.0),
        units: row[8].get_string().unwrap_or("").to_string(),
        accumulate: row[10].to_string() == "1",
        subfields: Vec::new(),
        components,
        comment,
    }
}

fn process_messages(sheet: Range<DataType>) -> Vec<MessageDefinition> {
    let mut rows = sheet.rows().skip(2);
    let mut messages: Vec<MessageDefinition> = Vec::new();
    let mut msg: MessageDefinition;
    let mut field: MessageFieldDefinition;
    let mut last_def_number: u8 = 0;

    // parse first message row to initialize first message to prevent unitialized compile error in loop
    let row = rows.next().unwrap();
    if let Some(v) = row[0].get_string() {
        msg = MessageDefinition::new(v, row[13].get_string().map(|v| v.to_string()));
    } else {
        panic!("Message name must be a string row={:?}.", row);
    }

    // process messages and fields
    for row in rows {
        // begin new message function
        if !row[0].is_empty() {
            if let Some(v) = row[0].get_string() {
                messages.push(msg);
                msg = MessageDefinition::new(v, row[13].get_string().map(|v| v.to_string()));
            } else {
                panic!("Message name must be a string row={:?}.", row);
            }
        } else if !row[1].is_empty() {
            field = new_message_field_definition(row);
            last_def_number = field.def_number;
            msg.field_map.insert(field.def_number, field);
        } else if !row[2].is_empty() {
            // process sub field
            let parent = msg
                .field_map
                .get_mut(&last_def_number)
                .expect("No parent field defined for subfield!");
            let mut temp_row: Vec<DataType> = Vec::from(row);
            temp_row[1] = DataType::Int(last_def_number as i64);
            field = new_message_field_definition(&temp_row);
            // store subfield ref_field, ref_field_value and defintion, if multiple values can
            // trigger this subfield we simply duplicate them
            let ref_field_names = row[11].get_string().expect("No reference field name(s)");
            let ref_field_vals = row[12].get_string().expect("No reference field value(s)");
            for (name, value) in
                split_csv_string!(ref_field_names).zip(split_csv_string!(ref_field_vals))
            {
                parent.subfields.push((name, value, field.clone()));
            }
        }
    }
    messages.push(msg);

    messages
}

pub fn parse_profile(
    profile_fname: &str,
    version: String,
) -> Result<FitProfile, Box<dyn std::error::Error>> {
    let mut excel: Xlsx<_> = open_workbook(&profile_fname)?;

    // process Types sheet
    let field_types = if let Some(Ok(sheet)) = excel.worksheet_range("Types") {
        process_types(sheet)
    } else {
        panic!("Could not access workbook sheet 'Types'");
    };

    // process Messages sheet
    let messages = if let Some(Ok(sheet)) = excel.worksheet_range("Messages") {
        process_messages(sheet)
    } else {
        panic!("Could not access workbook sheet 'Messages'");
    };

    Ok(FitProfile {
        version,
        field_types,
        messages,
    })
}

fn create_mesg_num_to_mesg_info_fn(
    messages: &[MessageDefinition],
    out: &mut File,
) -> Result<(), std::io::Error> {
    writeln!(out, "impl MesgNum {{")?;
    writeln!(out, "    pub fn message_info(self) -> MessageInfo {{")?;
    writeln!(out, "        match self {{")?;
    for msg in messages {
        writeln!(
            out,
            "            MesgNum::{} => {}(),",
            titlecase_string(&msg.name),
            msg.function_name()
        )?;
    }
    writeln!(out, "            _ => unknown_message(self),")?;
    writeln!(out, "        }}")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;

    Ok(())
}

/// call rustfmt on a generated file to cleanup auto-gen code
fn rustfmt(fname: &str) {
    Command::new("rustfmt")
        .arg(&fname)
        .status()
        .unwrap_or_else(|_| panic!("failed to execute rustfmt on {}", fname));
}

fn write_types_files(profile: &FitProfile) -> Result<(), std::io::Error> {
    let fname = "src/profile/field_types.rs";
    let mut out = File::create(&fname)?;

    writeln!(
        out,
        "//! Auto generated profile field types from FIT SDK Release: {}",
        profile.version
    )?;
    writeln!(
        out,
        "//! Not all of these may be used by the defined set of FIT messages"
    )?;
    writeln!(out, "#![allow(missing_docs)]")?;
    writeln!(out, "#![allow(dead_code)]")?;
    writeln!(out, "#![allow(clippy::unreadable_literal)]")?;
    writeln!(out, "use serde::Serialize;")?;
    writeln!(out, "use serde::ser::Serializer;")?;
    writeln!(out, "use std::convert;")?;
    writeln!(out, "use std::fmt;")?;

    // output enums and implementations
    for field_type in &profile.field_types {
        if !field_type.variant_map.is_empty() {
            field_type.generate_enum(&mut out)?;
        }
    }
    generate_main_field_type_enum(&profile.field_types, &mut out)?;

    rustfmt(fname);
    Ok(())
}

fn write_messages_file(profile: &FitProfile) -> Result<(), std::io::Error> {
    let fname = "src/profile/messages.rs";
    let mut out = File::create(&fname)?;

    writeln!(
        out,
        "//! Auto generated profile messages from FIT SDK Release: {}",
        profile.version
    )?;
    writeln!(out, "#![allow(missing_docs)]")?;
    writeln!(out, "#![allow(clippy::redundant_field_names)]")?;
    writeln!(out, "#![allow(clippy::unreadable_literal)]")?;
    writeln!(out, "use std::collections::HashMap;")?;
    writeln!(
        out,
        "use super::{{ComponentFieldInfo, FieldDataType, FieldInfo, MessageInfo}};"
    )?;
    writeln!(out, "use super::field_types::*;")?;
    writeln!(out, "pub const VERSION: &str = \"{}\";", profile.version)?;

    // output all message functions
    for msg in &profile.messages {
        msg.write_function_def(&mut out)?;
    }

    // output an unknown_message() fn that has no fields
    writeln!(
        out,
        "fn unknown_message(global_message_number: MesgNum) -> MessageInfo {{"
    )?;
    writeln!(out, "    MessageInfo {{")?;
    writeln!(out, "        name: \"unknown\",")?;
    writeln!(out, "        global_message_number,")?;
    writeln!(out, "        fields: HashMap::new()")?;
    writeln!(out, "    }}")?;
    writeln!(out, "}}")?;

    // output MesgNum implementation to allow parser to fetch info
    create_mesg_num_to_mesg_info_fn(&profile.messages, &mut out)?;

    rustfmt(fname);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // FIT_PROFILE should be set to the profile.xlsx file path
    println!("cargo:rerun-if-env-changed=FIT_PROFILE");
    let profile_fname = match env::var("FIT_PROFILE") {
        Ok(val) => {
            eprintln!("Reading FIT profile at {}", &val);
            val
        }
        Err(_) => {
            println!("cargo:warning=Did not update FIT profile, could not read FIT_PROFILE environment variable");
            return Ok(());
        }
    };
    let profile_vers = match env::var("FIT_PROFILE_VERSION") {
        Ok(val) => {
            eprintln!("Setting profile version to {}", &val);
            val
        }
        Err(e) => {
            println!("cargo:error=Did not update FIT profile, could not read FIT_PROFILE_VERSION environment variable");
            return Err(e.into());
        }
    };

    // process excel file and output
    let profile = parse_profile(&profile_fname, profile_vers).unwrap();
    write_types_files(&profile)?;
    write_messages_file(&profile)?;

    Ok(())
}
