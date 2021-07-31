//! Auto generated profile messages from FIT SDK Release: 21.60.00
#![allow(missing_docs)]
#![allow(clippy::redundant_field_names)]
#![allow(clippy::unreadable_literal)]
use super::field_types::*;
use super::{ComponentFieldInfo, FieldDataType, FieldInfo, MessageInfo};
use std::collections::HashMap;
pub const VERSION: &str = "21.60.00";
/// Must be first message in file.
pub fn file_id_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "type",
        field_type: FieldDataType::File,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "manufacturer",
        field_type: FieldDataType::Manufacturer,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "favero_product",
        field_type: FieldDataType::FaveroProduct,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, Manufacturer::FaveroElectronics.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, Manufacturer::Garmin.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, Manufacturer::Dynastream.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, Manufacturer::DynastreamOem.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, Manufacturer::Tacx.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "product",
        field_type: FieldDataType::UInt16,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "serial_number",
        field_type: FieldDataType::UInt32z,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Only set for files that are can be created/erased.
    let field = FieldInfo {
        name: "time_created",
        field_type: FieldDataType::DateTime,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // Only set for files that are not created/erased.
    let field = FieldInfo {
        name: "number",
        field_type: FieldDataType::UInt16,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // Optional free form string to indicate the devices name or model
    let field = FieldInfo {
        name: "product_name",
        field_type: FieldDataType::String,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    MessageInfo {
        name: "file_id",
        global_message_number: MesgNum::FileId,
        fields,
    }
}
pub fn file_creator_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "software_version",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "hardware_version",
        field_type: FieldDataType::UInt8,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    MessageInfo {
        name: "file_creator",
        global_message_number: MesgNum::FileCreator,
        fields,
    }
}
pub fn timestamp_correlation_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Fractional part of the UTC timestamp at the time the system timestamp was recorded.
    let field = FieldInfo {
        name: "fractional_timestamp",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 32768.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // Whole second part of the system timestamp
    let field = FieldInfo {
        name: "system_timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Fractional part of the system timestamp
    let field = FieldInfo {
        name: "fractional_system_timestamp",
        field_type: FieldDataType::UInt16,
        def_number: 2,
        scale: 32768.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // timestamp epoch expressed in local time used to convert timestamps to local time
    let field = FieldInfo {
        name: "local_timestamp",
        field_type: FieldDataType::LocalDateTime,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Millisecond part of the UTC timestamp at the time the system timestamp was recorded.
    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: FieldDataType::UInt16,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // Millisecond part of the system timestamp
    let field = FieldInfo {
        name: "system_timestamp_ms",
        field_type: FieldDataType::UInt16,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // Whole second part of UTC timestamp at the time the system timestamp was recorded.
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "timestamp_correlation",
        global_message_number: MesgNum::TimestampCorrelation,
        fields,
    }
}
pub fn software_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "version",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 100.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "part_number",
        field_type: FieldDataType::String,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "software",
        global_message_number: MesgNum::Software,
        fields,
    }
}
pub fn slave_device_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "manufacturer",
        field_type: FieldDataType::Manufacturer,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "favero_product",
        field_type: FieldDataType::FaveroProduct,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Manufacturer::FaveroElectronics.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Manufacturer::Garmin.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Manufacturer::Dynastream.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Manufacturer::DynastreamOem.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Manufacturer::Tacx.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "product",
        field_type: FieldDataType::UInt16,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(1, field);
    MessageInfo {
        name: "slave_device",
        global_message_number: MesgNum::SlaveDevice,
        fields,
    }
}
pub fn capabilities_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Use language_bits_x types where x is index of array.
    let field = FieldInfo {
        name: "languages",
        field_type: FieldDataType::UInt8z,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // Use sport_bits_x types where x is index of array.
    let field = FieldInfo {
        name: "sports",
        field_type: FieldDataType::SportBits0,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "workouts_supported",
        field_type: FieldDataType::WorkoutCapabilities,
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(21, field);
    let field = FieldInfo {
        name: "connectivity_supported",
        field_type: FieldDataType::ConnectivityCapabilities,
        def_number: 23,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(23, field);
    MessageInfo {
        name: "capabilities",
        global_message_number: MesgNum::Capabilities,
        fields,
    }
}
pub fn file_capabilities_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "type",
        field_type: FieldDataType::File,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "flags",
        field_type: FieldDataType::FileFlags,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "directory",
        field_type: FieldDataType::String,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "max_count",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "max_size",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "bytes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "file_capabilities",
        global_message_number: MesgNum::FileCapabilities,
        fields,
    }
}
pub fn mesg_capabilities_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "file",
        field_type: FieldDataType::File,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "mesg_num",
        field_type: FieldDataType::MesgNum,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "count_type",
        field_type: FieldDataType::MesgCount,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "num_per_file",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((2, MesgCount::NumPerFile.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "max_per_file",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((2, MesgCount::MaxPerFile.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "max_per_file_type",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((2, MesgCount::MaxPerFileType.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "count",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "mesg_capabilities",
        global_message_number: MesgNum::MesgCapabilities,
        fields,
    }
}
pub fn field_capabilities_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "file",
        field_type: FieldDataType::File,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "mesg_num",
        field_type: FieldDataType::MesgNum,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "field_num",
        field_type: FieldDataType::UInt8,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "count",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "field_capabilities",
        global_message_number: MesgNum::FieldCapabilities,
        fields,
    }
}
pub fn device_settings_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Index into time zone arrays.
    let field = FieldInfo {
        name: "active_time_zone",
        field_type: FieldDataType::UInt8,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // Offset from system time. Required to convert timestamp from system time to UTC.
    let field = FieldInfo {
        name: "utc_offset",
        field_type: FieldDataType::UInt32,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Offset from system time.
    let field = FieldInfo {
        name: "time_offset",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // Display mode for the time
    let field = FieldInfo {
        name: "time_mode",
        field_type: FieldDataType::TimeMode,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // timezone offset in 1/4 hour increments
    let field = FieldInfo {
        name: "time_zone_offset",
        field_type: FieldDataType::SInt8,
        def_number: 5,
        scale: 4.000000,
        offset: 0.000000,
        units: "hr",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // Mode for backlight
    let field = FieldInfo {
        name: "backlight_mode",
        field_type: FieldDataType::BacklightMode,
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(12, field);
    // Enabled state of the activity tracker functionality
    let field = FieldInfo {
        name: "activity_tracker_enabled",
        field_type: FieldDataType::Bool,
        def_number: 36,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(36, field);
    // UTC timestamp used to set the devices clock and date
    let field = FieldInfo {
        name: "clock_time",
        field_type: FieldDataType::DateTime,
        def_number: 39,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(39, field);
    // Bitfield  to configure enabled screens for each supported loop
    let field = FieldInfo {
        name: "pages_enabled",
        field_type: FieldDataType::UInt16,
        def_number: 40,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(40, field);
    // Enabled state of the move alert
    let field = FieldInfo {
        name: "move_alert_enabled",
        field_type: FieldDataType::Bool,
        def_number: 46,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(46, field);
    // Display mode for the date
    let field = FieldInfo {
        name: "date_mode",
        field_type: FieldDataType::DateMode,
        def_number: 47,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(47, field);
    let field = FieldInfo {
        name: "display_orientation",
        field_type: FieldDataType::DisplayOrientation,
        def_number: 55,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(55, field);
    let field = FieldInfo {
        name: "mounting_side",
        field_type: FieldDataType::Side,
        def_number: 56,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(56, field);
    // Bitfield to indicate one page as default for each supported loop
    let field = FieldInfo {
        name: "default_page",
        field_type: FieldDataType::UInt16,
        def_number: 57,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(57, field);
    // Minimum steps before an autosync can occur
    let field = FieldInfo {
        name: "autosync_min_steps",
        field_type: FieldDataType::UInt16,
        def_number: 58,
        scale: 1.000000,
        offset: 0.000000,
        units: "steps",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(58, field);
    // Minimum minutes before an autosync can occur
    let field = FieldInfo {
        name: "autosync_min_time",
        field_type: FieldDataType::UInt16,
        def_number: 59,
        scale: 1.000000,
        offset: 0.000000,
        units: "minutes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(59, field);
    // Enable auto-detect setting for the lactate threshold feature.
    let field = FieldInfo {
        name: "lactate_threshold_autodetect_enabled",
        field_type: FieldDataType::Bool,
        def_number: 80,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(80, field);
    // Automatically upload using BLE
    let field = FieldInfo {
        name: "ble_auto_upload_enabled",
        field_type: FieldDataType::Bool,
        def_number: 86,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(86, field);
    // Helps to conserve battery by changing modes
    let field = FieldInfo {
        name: "auto_sync_frequency",
        field_type: FieldDataType::AutoSyncFrequency,
        def_number: 89,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(89, field);
    // Allows setting specific activities auto-activity detect enabled/disabled settings
    let field = FieldInfo {
        name: "auto_activity_detect",
        field_type: FieldDataType::AutoActivityDetect,
        def_number: 90,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(90, field);
    // Number of screens configured to display
    let field = FieldInfo {
        name: "number_of_screens",
        field_type: FieldDataType::UInt8,
        def_number: 94,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(94, field);
    // Smart Notification display orientation
    let field = FieldInfo {
        name: "smart_notification_display_orientation",
        field_type: FieldDataType::DisplayOrientation,
        def_number: 95,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(95, field);
    let field = FieldInfo {
        name: "tap_interface",
        field_type: FieldDataType::Switch,
        def_number: 134,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(134, field);
    // Used to hold the tap threshold setting
    let field = FieldInfo {
        name: "tap_sensitivity",
        field_type: FieldDataType::TapSensitivity,
        def_number: 174,
        scale: 1.000000,
        offset: 1.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(174, field);
    MessageInfo {
        name: "device_settings",
        global_message_number: MesgNum::DeviceSettings,
        fields,
    }
}
pub fn user_profile_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "friendly_name",
        field_type: FieldDataType::String,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "gender",
        field_type: FieldDataType::Gender,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "age",
        field_type: FieldDataType::UInt8,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "years",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "height",
        field_type: FieldDataType::UInt8,
        def_number: 3,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "weight",
        field_type: FieldDataType::UInt16,
        def_number: 4,
        scale: 10.000000,
        offset: 0.000000,
        units: "kg",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "language",
        field_type: FieldDataType::Language,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "elev_setting",
        field_type: FieldDataType::DisplayMeasure,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "weight_setting",
        field_type: FieldDataType::DisplayMeasure,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    let field = FieldInfo {
        name: "resting_heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "default_max_running_heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "default_max_biking_heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "default_max_heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    let field = FieldInfo {
        name: "hr_setting",
        field_type: FieldDataType::DisplayHeart,
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(12, field);
    let field = FieldInfo {
        name: "speed_setting",
        field_type: FieldDataType::DisplayMeasure,
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(13, field);
    let field = FieldInfo {
        name: "dist_setting",
        field_type: FieldDataType::DisplayMeasure,
        def_number: 14,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(14, field);
    let field = FieldInfo {
        name: "power_setting",
        field_type: FieldDataType::DisplayPower,
        def_number: 16,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(16, field);
    let field = FieldInfo {
        name: "activity_class",
        field_type: FieldDataType::ActivityClass,
        def_number: 17,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(17, field);
    let field = FieldInfo {
        name: "position_setting",
        field_type: FieldDataType::DisplayPosition,
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(18, field);
    let field = FieldInfo {
        name: "temperature_setting",
        field_type: FieldDataType::DisplayMeasure,
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(21, field);
    let field = FieldInfo {
        name: "local_id",
        field_type: FieldDataType::UserLocalId,
        def_number: 22,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(22, field);
    let field = FieldInfo {
        name: "global_id",
        field_type: FieldDataType::Byte,
        def_number: 23,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(23, field);
    // Typical wake time
    let field = FieldInfo {
        name: "wake_time",
        field_type: FieldDataType::LocaltimeIntoDay,
        def_number: 28,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(28, field);
    // Typical bed time
    let field = FieldInfo {
        name: "sleep_time",
        field_type: FieldDataType::LocaltimeIntoDay,
        def_number: 29,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(29, field);
    let field = FieldInfo {
        name: "height_setting",
        field_type: FieldDataType::DisplayMeasure,
        def_number: 30,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(30, field);
    // User defined running step length set to 0 for auto length
    let field = FieldInfo {
        name: "user_running_step_length",
        field_type: FieldDataType::UInt16,
        def_number: 31,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(31, field);
    // User defined walking step length set to 0 for auto length
    let field = FieldInfo {
        name: "user_walking_step_length",
        field_type: FieldDataType::UInt16,
        def_number: 32,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(32, field);
    let field = FieldInfo {
        name: "depth_setting",
        field_type: FieldDataType::DisplayMeasure,
        def_number: 47,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(47, field);
    let field = FieldInfo {
        name: "dive_count",
        field_type: FieldDataType::UInt32,
        def_number: 49,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(49, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "user_profile",
        global_message_number: MesgNum::UserProfile,
        fields,
    }
}
pub fn hrm_profile_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "enabled",
        field_type: FieldDataType::Bool,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "hrm_ant_id",
        field_type: FieldDataType::UInt16z,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "log_hrv",
        field_type: FieldDataType::Bool,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "hrm_ant_id_trans_type",
        field_type: FieldDataType::UInt8z,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "hrm_profile",
        global_message_number: MesgNum::HrmProfile,
        fields,
    }
}
pub fn sdm_profile_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "enabled",
        field_type: FieldDataType::Bool,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "sdm_ant_id",
        field_type: FieldDataType::UInt16z,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "sdm_cal_factor",
        field_type: FieldDataType::UInt16,
        def_number: 2,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "odometer",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Use footpod for speed source instead of GPS
    let field = FieldInfo {
        name: "speed_source",
        field_type: FieldDataType::Bool,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "sdm_ant_id_trans_type",
        field_type: FieldDataType::UInt8z,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // Rollover counter that can be used to extend the odometer
    let field = FieldInfo {
        name: "odometer_rollover",
        field_type: FieldDataType::UInt8,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "sdm_profile",
        global_message_number: MesgNum::SdmProfile,
        fields,
    }
}
pub fn bike_profile_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "name",
        field_type: FieldDataType::String,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "sport",
        field_type: FieldDataType::Sport,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "sub_sport",
        field_type: FieldDataType::SubSport,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "odometer",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "bike_spd_ant_id",
        field_type: FieldDataType::UInt16z,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "bike_cad_ant_id",
        field_type: FieldDataType::UInt16z,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "bike_spdcad_ant_id",
        field_type: FieldDataType::UInt16z,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "bike_power_ant_id",
        field_type: FieldDataType::UInt16z,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    let field = FieldInfo {
        name: "custom_wheelsize",
        field_type: FieldDataType::UInt16,
        def_number: 8,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "auto_wheelsize",
        field_type: FieldDataType::UInt16,
        def_number: 9,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "bike_weight",
        field_type: FieldDataType::UInt16,
        def_number: 10,
        scale: 10.000000,
        offset: 0.000000,
        units: "kg",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "power_cal_factor",
        field_type: FieldDataType::UInt16,
        def_number: 11,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    let field = FieldInfo {
        name: "auto_wheel_cal",
        field_type: FieldDataType::Bool,
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(12, field);
    let field = FieldInfo {
        name: "auto_power_zero",
        field_type: FieldDataType::Bool,
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(13, field);
    let field = FieldInfo {
        name: "id",
        field_type: FieldDataType::UInt8,
        def_number: 14,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(14, field);
    let field = FieldInfo {
        name: "spd_enabled",
        field_type: FieldDataType::Bool,
        def_number: 15,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(15, field);
    let field = FieldInfo {
        name: "cad_enabled",
        field_type: FieldDataType::Bool,
        def_number: 16,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(16, field);
    let field = FieldInfo {
        name: "spdcad_enabled",
        field_type: FieldDataType::Bool,
        def_number: 17,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(17, field);
    let field = FieldInfo {
        name: "power_enabled",
        field_type: FieldDataType::Bool,
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(18, field);
    let field = FieldInfo {
        name: "crank_length",
        field_type: FieldDataType::UInt8,
        def_number: 19,
        scale: 2.000000,
        offset: 0.000000,
        units: "mm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(19, field);
    let field = FieldInfo {
        name: "enabled",
        field_type: FieldDataType::Bool,
        def_number: 20,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(20, field);
    let field = FieldInfo {
        name: "bike_spd_ant_id_trans_type",
        field_type: FieldDataType::UInt8z,
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(21, field);
    let field = FieldInfo {
        name: "bike_cad_ant_id_trans_type",
        field_type: FieldDataType::UInt8z,
        def_number: 22,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(22, field);
    let field = FieldInfo {
        name: "bike_spdcad_ant_id_trans_type",
        field_type: FieldDataType::UInt8z,
        def_number: 23,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(23, field);
    let field = FieldInfo {
        name: "bike_power_ant_id_trans_type",
        field_type: FieldDataType::UInt8z,
        def_number: 24,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(24, field);
    // Rollover counter that can be used to extend the odometer
    let field = FieldInfo {
        name: "odometer_rollover",
        field_type: FieldDataType::UInt8,
        def_number: 37,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(37, field);
    // Number of front gears
    let field = FieldInfo {
        name: "front_gear_num",
        field_type: FieldDataType::UInt8z,
        def_number: 38,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(38, field);
    // Number of teeth on each gear 0 is innermost
    let field = FieldInfo {
        name: "front_gear",
        field_type: FieldDataType::UInt8z,
        def_number: 39,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(39, field);
    // Number of rear gears
    let field = FieldInfo {
        name: "rear_gear_num",
        field_type: FieldDataType::UInt8z,
        def_number: 40,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(40, field);
    // Number of teeth on each gear 0 is innermost
    let field = FieldInfo {
        name: "rear_gear",
        field_type: FieldDataType::UInt8z,
        def_number: 41,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(41, field);
    let field = FieldInfo {
        name: "shimano_di2_enabled",
        field_type: FieldDataType::Bool,
        def_number: 44,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(44, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "bike_profile",
        global_message_number: MesgNum::BikeProfile,
        fields,
    }
}
pub fn connectivity_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Use Bluetooth for connectivity features
    let field = FieldInfo {
        name: "bluetooth_enabled",
        field_type: FieldDataType::Bool,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // Use Bluetooth Low Energy for connectivity features
    let field = FieldInfo {
        name: "bluetooth_le_enabled",
        field_type: FieldDataType::Bool,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Use ANT for connectivity features
    let field = FieldInfo {
        name: "ant_enabled",
        field_type: FieldDataType::Bool,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "name",
        field_type: FieldDataType::String,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "live_tracking_enabled",
        field_type: FieldDataType::Bool,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "weather_conditions_enabled",
        field_type: FieldDataType::Bool,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "weather_alerts_enabled",
        field_type: FieldDataType::Bool,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "auto_activity_upload_enabled",
        field_type: FieldDataType::Bool,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    let field = FieldInfo {
        name: "course_download_enabled",
        field_type: FieldDataType::Bool,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "workout_download_enabled",
        field_type: FieldDataType::Bool,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "gps_ephemeris_download_enabled",
        field_type: FieldDataType::Bool,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "incident_detection_enabled",
        field_type: FieldDataType::Bool,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    let field = FieldInfo {
        name: "grouptrack_enabled",
        field_type: FieldDataType::Bool,
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(12, field);
    MessageInfo {
        name: "connectivity",
        global_message_number: MesgNum::Connectivity,
        fields,
    }
}
pub fn watchface_settings_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "mode",
        field_type: FieldDataType::WatchfaceMode,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "digital_layout",
        field_type: FieldDataType::DigitalWatchfaceLayout,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, WatchfaceMode::Digital.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "analog_layout",
        field_type: FieldDataType::AnalogWatchfaceLayout,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, WatchfaceMode::Analog.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "layout",
        field_type: FieldDataType::Byte,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "watchface_settings",
        global_message_number: MesgNum::WatchfaceSettings,
        fields,
    }
}
pub fn ohr_settings_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "enabled",
        field_type: FieldDataType::Switch,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "ohr_settings",
        global_message_number: MesgNum::OhrSettings,
        fields,
    }
}
pub fn zones_target_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "max_heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "threshold_heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "functional_threshold_power",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "hr_calc_type",
        field_type: FieldDataType::HrZoneCalc,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "pwr_calc_type",
        field_type: FieldDataType::PwrZoneCalc,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    MessageInfo {
        name: "zones_target",
        global_message_number: MesgNum::ZonesTarget,
        fields,
    }
}
pub fn sport_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "sport",
        field_type: FieldDataType::Sport,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "sub_sport",
        field_type: FieldDataType::SubSport,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "name",
        field_type: FieldDataType::String,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    MessageInfo {
        name: "sport",
        global_message_number: MesgNum::Sport,
        fields,
    }
}
pub fn hr_zone_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "high_bpm",
        field_type: FieldDataType::UInt8,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "name",
        field_type: FieldDataType::String,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "hr_zone",
        global_message_number: MesgNum::HrZone,
        fields,
    }
}
pub fn speed_zone_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "high_value",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "name",
        field_type: FieldDataType::String,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "speed_zone",
        global_message_number: MesgNum::SpeedZone,
        fields,
    }
}
pub fn cadence_zone_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "high_value",
        field_type: FieldDataType::UInt8,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "name",
        field_type: FieldDataType::String,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "cadence_zone",
        global_message_number: MesgNum::CadenceZone,
        fields,
    }
}
pub fn power_zone_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "high_value",
        field_type: FieldDataType::UInt16,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "name",
        field_type: FieldDataType::String,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "power_zone",
        global_message_number: MesgNum::PowerZone,
        fields,
    }
}
pub fn met_zone_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "high_bpm",
        field_type: FieldDataType::UInt8,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "calories",
        field_type: FieldDataType::UInt16,
        def_number: 2,
        scale: 10.000000,
        offset: 0.000000,
        units: "kcal / min",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "fat_calories",
        field_type: FieldDataType::UInt8,
        def_number: 3,
        scale: 10.000000,
        offset: 0.000000,
        units: "kcal / min",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "met_zone",
        global_message_number: MesgNum::MetZone,
        fields,
    }
}
pub fn dive_settings_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "name",
        field_type: FieldDataType::String,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "model",
        field_type: FieldDataType::TissueModelType,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "gf_low",
        field_type: FieldDataType::UInt8,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "gf_high",
        field_type: FieldDataType::UInt8,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "water_type",
        field_type: FieldDataType::WaterType,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // Fresh water is usually 1000; salt water is usually 1025
    let field = FieldInfo {
        name: "water_density",
        field_type: FieldDataType::Float32,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "kg/m^3",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // Typically 1.40
    let field = FieldInfo {
        name: "po2_warn",
        field_type: FieldDataType::UInt8,
        def_number: 6,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    // Typically 1.60
    let field = FieldInfo {
        name: "po2_critical",
        field_type: FieldDataType::UInt8,
        def_number: 7,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    let field = FieldInfo {
        name: "po2_deco",
        field_type: FieldDataType::UInt8,
        def_number: 8,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "safety_stop_enabled",
        field_type: FieldDataType::Bool,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "bottom_depth",
        field_type: FieldDataType::Float32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "bottom_time",
        field_type: FieldDataType::UInt32,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    let field = FieldInfo {
        name: "apnea_countdown_enabled",
        field_type: FieldDataType::Bool,
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(12, field);
    let field = FieldInfo {
        name: "apnea_countdown_time",
        field_type: FieldDataType::UInt32,
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(13, field);
    let field = FieldInfo {
        name: "backlight_mode",
        field_type: FieldDataType::DiveBacklightMode,
        def_number: 14,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(14, field);
    let field = FieldInfo {
        name: "backlight_brightness",
        field_type: FieldDataType::UInt8,
        def_number: 15,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(15, field);
    let field = FieldInfo {
        name: "backlight_timeout",
        field_type: FieldDataType::BacklightTimeout,
        def_number: 16,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(16, field);
    // Time between surfacing and ending the activity
    let field = FieldInfo {
        name: "repeat_dive_interval",
        field_type: FieldDataType::UInt16,
        def_number: 17,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(17, field);
    // Time at safety stop (if enabled)
    let field = FieldInfo {
        name: "safety_stop_time",
        field_type: FieldDataType::UInt16,
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(18, field);
    let field = FieldInfo {
        name: "heart_rate_source_type",
        field_type: FieldDataType::SourceType,
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(19, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "heart_rate_antplus_device_type",
        field_type: FieldDataType::AntplusDeviceType,
        def_number: 20,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((19, SourceType::Antplus.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "heart_rate_local_device_type",
        field_type: FieldDataType::LocalDeviceType,
        def_number: 20,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((19, SourceType::Local.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "heart_rate_source",
        field_type: FieldDataType::UInt8,
        def_number: 20,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(20, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "dive_settings",
        global_message_number: MesgNum::DiveSettings,
        fields,
    }
}
pub fn dive_alarm_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "depth",
        field_type: FieldDataType::UInt32,
        def_number: 0,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "time",
        field_type: FieldDataType::SInt32,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "enabled",
        field_type: FieldDataType::Bool,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "alarm_type",
        field_type: FieldDataType::DiveAlarmType,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "sound",
        field_type: FieldDataType::Tone,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "dive_types",
        field_type: FieldDataType::SubSport,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // Index of the alarm
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "dive_alarm",
        global_message_number: MesgNum::DiveAlarm,
        fields,
    }
}
pub fn dive_gas_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "helium_content",
        field_type: FieldDataType::UInt8,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "oxygen_content",
        field_type: FieldDataType::UInt8,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "status",
        field_type: FieldDataType::DiveGasStatus,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "dive_gas",
        global_message_number: MesgNum::DiveGas,
        fields,
    }
}
pub fn goal_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "sport",
        field_type: FieldDataType::Sport,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "sub_sport",
        field_type: FieldDataType::SubSport,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "start_date",
        field_type: FieldDataType::DateTime,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "end_date",
        field_type: FieldDataType::DateTime,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "type",
        field_type: FieldDataType::Goal,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "value",
        field_type: FieldDataType::UInt32,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "repeat",
        field_type: FieldDataType::Bool,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "target_value",
        field_type: FieldDataType::UInt32,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    let field = FieldInfo {
        name: "recurrence",
        field_type: FieldDataType::GoalRecurrence,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "recurrence_value",
        field_type: FieldDataType::UInt16,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "enabled",
        field_type: FieldDataType::Bool,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "source",
        field_type: FieldDataType::GoalSource,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "goal",
        global_message_number: MesgNum::Goal,
        fields,
    }
}
pub fn activity_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Exclude pauses
    let field = FieldInfo {
        name: "total_timer_time",
        field_type: FieldDataType::UInt32,
        def_number: 0,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "num_sessions",
        field_type: FieldDataType::UInt16,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "type",
        field_type: FieldDataType::Activity,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "event",
        field_type: FieldDataType::Event,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "event_type",
        field_type: FieldDataType::EventType,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // timestamp epoch expressed in local time, used to convert activity timestamps to local time
    let field = FieldInfo {
        name: "local_timestamp",
        field_type: FieldDataType::LocalDateTime,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "event_group",
        field_type: FieldDataType::UInt8,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "activity",
        global_message_number: MesgNum::Activity,
        fields,
    }
}
pub fn session_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // session
    let field = FieldInfo {
        name: "event",
        field_type: FieldDataType::Event,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // stop
    let field = FieldInfo {
        name: "event_type",
        field_type: FieldDataType::EventType,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "start_time",
        field_type: FieldDataType::DateTime,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "start_position_lat",
        field_type: FieldDataType::SInt32,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "start_position_long",
        field_type: FieldDataType::SInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "sport",
        field_type: FieldDataType::Sport,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "sub_sport",
        field_type: FieldDataType::SubSport,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    // Time (includes pauses)
    let field = FieldInfo {
        name: "total_elapsed_time",
        field_type: FieldDataType::UInt32,
        def_number: 7,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    // Timer Time (excludes pauses)
    let field = FieldInfo {
        name: "total_timer_time",
        field_type: FieldDataType::UInt32,
        def_number: 8,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "total_distance",
        field_type: FieldDataType::UInt32,
        def_number: 9,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "total_strides",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "strides",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((5, Sport::Running.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "total_strides",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "strides",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((5, Sport::Walking.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "total_strokes",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "strokes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((5, Sport::Cycling.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "total_strokes",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "strokes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((5, Sport::Swimming.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "total_strokes",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "strokes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((5, Sport::Rowing.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "total_strokes",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "strokes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((5, Sport::StandUpPaddleboarding.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "total_cycles",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "cycles",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "total_calories",
        field_type: FieldDataType::UInt16,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    let field = FieldInfo {
        name: "total_fat_calories",
        field_type: FieldDataType::UInt16,
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(13, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 124,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        bits: 16,
        accumulate: false,
    };
    components.push(comp_fld);
    // total_distance / total_timer_time
    let field = FieldInfo {
        name: "avg_speed",
        field_type: FieldDataType::UInt16,
        def_number: 14,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(14, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 125,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        bits: 16,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "max_speed",
        field_type: FieldDataType::UInt16,
        def_number: 15,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(15, field);
    // average heart rate (excludes pause time)
    let field = FieldInfo {
        name: "avg_heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 16,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(16, field);
    let field = FieldInfo {
        name: "max_heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 17,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(17, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "avg_running_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "strides/min",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((5, Sport::Running.as_i64(), sub_fld));
    // total_cycles / total_timer_time if non_zero_avg_cadence otherwise total_cycles / total_elapsed_time
    let field = FieldInfo {
        name: "avg_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(18, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "max_running_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "strides/min",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((5, Sport::Running.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "max_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(19, field);
    // total_power / total_timer_time if non_zero_avg_power otherwise total_power / total_elapsed_time
    let field = FieldInfo {
        name: "avg_power",
        field_type: FieldDataType::UInt16,
        def_number: 20,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(20, field);
    let field = FieldInfo {
        name: "max_power",
        field_type: FieldDataType::UInt16,
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(21, field);
    let field = FieldInfo {
        name: "total_ascent",
        field_type: FieldDataType::UInt16,
        def_number: 22,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(22, field);
    let field = FieldInfo {
        name: "total_descent",
        field_type: FieldDataType::UInt16,
        def_number: 23,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(23, field);
    let field = FieldInfo {
        name: "total_training_effect",
        field_type: FieldDataType::UInt8,
        def_number: 24,
        scale: 10.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(24, field);
    let field = FieldInfo {
        name: "first_lap_index",
        field_type: FieldDataType::UInt16,
        def_number: 25,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(25, field);
    let field = FieldInfo {
        name: "num_laps",
        field_type: FieldDataType::UInt16,
        def_number: 26,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(26, field);
    let field = FieldInfo {
        name: "event_group",
        field_type: FieldDataType::UInt8,
        def_number: 27,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(27, field);
    let field = FieldInfo {
        name: "trigger",
        field_type: FieldDataType::SessionTrigger,
        def_number: 28,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(28, field);
    // North east corner latitude
    let field = FieldInfo {
        name: "nec_lat",
        field_type: FieldDataType::SInt32,
        def_number: 29,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(29, field);
    // North east corner longitude
    let field = FieldInfo {
        name: "nec_long",
        field_type: FieldDataType::SInt32,
        def_number: 30,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(30, field);
    // South west corner latitude
    let field = FieldInfo {
        name: "swc_lat",
        field_type: FieldDataType::SInt32,
        def_number: 31,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(31, field);
    // South west corner longitude
    let field = FieldInfo {
        name: "swc_long",
        field_type: FieldDataType::SInt32,
        def_number: 32,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(32, field);
    // # of lengths of swim pool
    let field = FieldInfo {
        name: "num_lengths",
        field_type: FieldDataType::UInt16,
        def_number: 33,
        scale: 1.000000,
        offset: 0.000000,
        units: "lengths",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(33, field);
    let field = FieldInfo {
        name: "normalized_power",
        field_type: FieldDataType::UInt16,
        def_number: 34,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(34, field);
    let field = FieldInfo {
        name: "training_stress_score",
        field_type: FieldDataType::UInt16,
        def_number: 35,
        scale: 10.000000,
        offset: 0.000000,
        units: "tss",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(35, field);
    let field = FieldInfo {
        name: "intensity_factor",
        field_type: FieldDataType::UInt16,
        def_number: 36,
        scale: 1000.000000,
        offset: 0.000000,
        units: "if",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(36, field);
    let field = FieldInfo {
        name: "left_right_balance",
        field_type: FieldDataType::LeftRightBalance100,
        def_number: 37,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(37, field);
    let field = FieldInfo {
        name: "avg_stroke_count",
        field_type: FieldDataType::UInt32,
        def_number: 41,
        scale: 10.000000,
        offset: 0.000000,
        units: "strokes/lap",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(41, field);
    let field = FieldInfo {
        name: "avg_stroke_distance",
        field_type: FieldDataType::UInt16,
        def_number: 42,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(42, field);
    let field = FieldInfo {
        name: "swim_stroke",
        field_type: FieldDataType::SwimStroke,
        def_number: 43,
        scale: 1.000000,
        offset: 0.000000,
        units: "swim_stroke",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(43, field);
    let field = FieldInfo {
        name: "pool_length",
        field_type: FieldDataType::UInt16,
        def_number: 44,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(44, field);
    let field = FieldInfo {
        name: "threshold_power",
        field_type: FieldDataType::UInt16,
        def_number: 45,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(45, field);
    let field = FieldInfo {
        name: "pool_length_unit",
        field_type: FieldDataType::DisplayMeasure,
        def_number: 46,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(46, field);
    // # of active lengths of swim pool
    let field = FieldInfo {
        name: "num_active_lengths",
        field_type: FieldDataType::UInt16,
        def_number: 47,
        scale: 1.000000,
        offset: 0.000000,
        units: "lengths",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(47, field);
    let field = FieldInfo {
        name: "total_work",
        field_type: FieldDataType::UInt32,
        def_number: 48,
        scale: 1.000000,
        offset: 0.000000,
        units: "J",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(48, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 126,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        bits: 16,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "avg_altitude",
        field_type: FieldDataType::UInt16,
        def_number: 49,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(49, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 128,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        bits: 16,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "max_altitude",
        field_type: FieldDataType::UInt16,
        def_number: 50,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(50, field);
    let field = FieldInfo {
        name: "gps_accuracy",
        field_type: FieldDataType::UInt8,
        def_number: 51,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(51, field);
    let field = FieldInfo {
        name: "avg_grade",
        field_type: FieldDataType::SInt16,
        def_number: 52,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(52, field);
    let field = FieldInfo {
        name: "avg_pos_grade",
        field_type: FieldDataType::SInt16,
        def_number: 53,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(53, field);
    let field = FieldInfo {
        name: "avg_neg_grade",
        field_type: FieldDataType::SInt16,
        def_number: 54,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(54, field);
    let field = FieldInfo {
        name: "max_pos_grade",
        field_type: FieldDataType::SInt16,
        def_number: 55,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(55, field);
    let field = FieldInfo {
        name: "max_neg_grade",
        field_type: FieldDataType::SInt16,
        def_number: 56,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(56, field);
    let field = FieldInfo {
        name: "avg_temperature",
        field_type: FieldDataType::SInt8,
        def_number: 57,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(57, field);
    let field = FieldInfo {
        name: "max_temperature",
        field_type: FieldDataType::SInt8,
        def_number: 58,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(58, field);
    let field = FieldInfo {
        name: "total_moving_time",
        field_type: FieldDataType::UInt32,
        def_number: 59,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(59, field);
    let field = FieldInfo {
        name: "avg_pos_vertical_speed",
        field_type: FieldDataType::SInt16,
        def_number: 60,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(60, field);
    let field = FieldInfo {
        name: "avg_neg_vertical_speed",
        field_type: FieldDataType::SInt16,
        def_number: 61,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(61, field);
    let field = FieldInfo {
        name: "max_pos_vertical_speed",
        field_type: FieldDataType::SInt16,
        def_number: 62,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(62, field);
    let field = FieldInfo {
        name: "max_neg_vertical_speed",
        field_type: FieldDataType::SInt16,
        def_number: 63,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(63, field);
    let field = FieldInfo {
        name: "min_heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 64,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(64, field);
    let field = FieldInfo {
        name: "time_in_hr_zone",
        field_type: FieldDataType::UInt32,
        def_number: 65,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(65, field);
    let field = FieldInfo {
        name: "time_in_speed_zone",
        field_type: FieldDataType::UInt32,
        def_number: 66,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(66, field);
    let field = FieldInfo {
        name: "time_in_cadence_zone",
        field_type: FieldDataType::UInt32,
        def_number: 67,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(67, field);
    let field = FieldInfo {
        name: "time_in_power_zone",
        field_type: FieldDataType::UInt32,
        def_number: 68,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(68, field);
    let field = FieldInfo {
        name: "avg_lap_time",
        field_type: FieldDataType::UInt32,
        def_number: 69,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(69, field);
    let field = FieldInfo {
        name: "best_lap_index",
        field_type: FieldDataType::UInt16,
        def_number: 70,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(70, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 127,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        bits: 16,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "min_altitude",
        field_type: FieldDataType::UInt16,
        def_number: 71,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(71, field);
    let field = FieldInfo {
        name: "player_score",
        field_type: FieldDataType::UInt16,
        def_number: 82,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(82, field);
    let field = FieldInfo {
        name: "opponent_score",
        field_type: FieldDataType::UInt16,
        def_number: 83,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(83, field);
    let field = FieldInfo {
        name: "opponent_name",
        field_type: FieldDataType::String,
        def_number: 84,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(84, field);
    // stroke_type enum used as the index
    let field = FieldInfo {
        name: "stroke_count",
        field_type: FieldDataType::UInt16,
        def_number: 85,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(85, field);
    // zone number used as the index
    let field = FieldInfo {
        name: "zone_count",
        field_type: FieldDataType::UInt16,
        def_number: 86,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(86, field);
    let field = FieldInfo {
        name: "max_ball_speed",
        field_type: FieldDataType::UInt16,
        def_number: 87,
        scale: 100.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(87, field);
    let field = FieldInfo {
        name: "avg_ball_speed",
        field_type: FieldDataType::UInt16,
        def_number: 88,
        scale: 100.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(88, field);
    let field = FieldInfo {
        name: "avg_vertical_oscillation",
        field_type: FieldDataType::UInt16,
        def_number: 89,
        scale: 10.000000,
        offset: 0.000000,
        units: "mm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(89, field);
    let field = FieldInfo {
        name: "avg_stance_time_percent",
        field_type: FieldDataType::UInt16,
        def_number: 90,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(90, field);
    let field = FieldInfo {
        name: "avg_stance_time",
        field_type: FieldDataType::UInt16,
        def_number: 91,
        scale: 10.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(91, field);
    // fractional part of the avg_cadence
    let field = FieldInfo {
        name: "avg_fractional_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 92,
        scale: 128.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(92, field);
    // fractional part of the max_cadence
    let field = FieldInfo {
        name: "max_fractional_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 93,
        scale: 128.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(93, field);
    // fractional part of the total_cycles
    let field = FieldInfo {
        name: "total_fractional_cycles",
        field_type: FieldDataType::UInt8,
        def_number: 94,
        scale: 128.000000,
        offset: 0.000000,
        units: "cycles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(94, field);
    // Avg saturated and unsaturated hemoglobin
    let field = FieldInfo {
        name: "avg_total_hemoglobin_conc",
        field_type: FieldDataType::UInt16,
        def_number: 95,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(95, field);
    // Min saturated and unsaturated hemoglobin
    let field = FieldInfo {
        name: "min_total_hemoglobin_conc",
        field_type: FieldDataType::UInt16,
        def_number: 96,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(96, field);
    // Max saturated and unsaturated hemoglobin
    let field = FieldInfo {
        name: "max_total_hemoglobin_conc",
        field_type: FieldDataType::UInt16,
        def_number: 97,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(97, field);
    // Avg percentage of hemoglobin saturated with oxygen
    let field = FieldInfo {
        name: "avg_saturated_hemoglobin_percent",
        field_type: FieldDataType::UInt16,
        def_number: 98,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(98, field);
    // Min percentage of hemoglobin saturated with oxygen
    let field = FieldInfo {
        name: "min_saturated_hemoglobin_percent",
        field_type: FieldDataType::UInt16,
        def_number: 99,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(99, field);
    // Max percentage of hemoglobin saturated with oxygen
    let field = FieldInfo {
        name: "max_saturated_hemoglobin_percent",
        field_type: FieldDataType::UInt16,
        def_number: 100,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(100, field);
    let field = FieldInfo {
        name: "avg_left_torque_effectiveness",
        field_type: FieldDataType::UInt8,
        def_number: 101,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(101, field);
    let field = FieldInfo {
        name: "avg_right_torque_effectiveness",
        field_type: FieldDataType::UInt8,
        def_number: 102,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(102, field);
    let field = FieldInfo {
        name: "avg_left_pedal_smoothness",
        field_type: FieldDataType::UInt8,
        def_number: 103,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(103, field);
    let field = FieldInfo {
        name: "avg_right_pedal_smoothness",
        field_type: FieldDataType::UInt8,
        def_number: 104,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(104, field);
    let field = FieldInfo {
        name: "avg_combined_pedal_smoothness",
        field_type: FieldDataType::UInt8,
        def_number: 105,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(105, field);
    let field = FieldInfo {
        name: "sport_index",
        field_type: FieldDataType::UInt8,
        def_number: 111,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(111, field);
    // Total time spend in the standing position
    let field = FieldInfo {
        name: "time_standing",
        field_type: FieldDataType::UInt32,
        def_number: 112,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(112, field);
    // Number of transitions to the standing state
    let field = FieldInfo {
        name: "stand_count",
        field_type: FieldDataType::UInt16,
        def_number: 113,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(113, field);
    // Average platform center offset Left
    let field = FieldInfo {
        name: "avg_left_pco",
        field_type: FieldDataType::SInt8,
        def_number: 114,
        scale: 1.000000,
        offset: 0.000000,
        units: "mm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(114, field);
    // Average platform center offset Right
    let field = FieldInfo {
        name: "avg_right_pco",
        field_type: FieldDataType::SInt8,
        def_number: 115,
        scale: 1.000000,
        offset: 0.000000,
        units: "mm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(115, field);
    // Average left power phase angles. Indexes defined by power_phase_type.
    let field = FieldInfo {
        name: "avg_left_power_phase",
        field_type: FieldDataType::UInt8,
        def_number: 116,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(116, field);
    // Average left power phase peak angles. Data value indexes defined by power_phase_type.
    let field = FieldInfo {
        name: "avg_left_power_phase_peak",
        field_type: FieldDataType::UInt8,
        def_number: 117,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(117, field);
    // Average right power phase angles. Data value indexes defined by power_phase_type.
    let field = FieldInfo {
        name: "avg_right_power_phase",
        field_type: FieldDataType::UInt8,
        def_number: 118,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(118, field);
    // Average right power phase peak angles data value indexes  defined by power_phase_type.
    let field = FieldInfo {
        name: "avg_right_power_phase_peak",
        field_type: FieldDataType::UInt8,
        def_number: 119,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(119, field);
    // Average power by position. Data value indexes defined by rider_position_type.
    let field = FieldInfo {
        name: "avg_power_position",
        field_type: FieldDataType::UInt16,
        def_number: 120,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(120, field);
    // Maximum power by position. Data value indexes defined by rider_position_type.
    let field = FieldInfo {
        name: "max_power_position",
        field_type: FieldDataType::UInt16,
        def_number: 121,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(121, field);
    // Average cadence by position. Data value indexes defined by rider_position_type.
    let field = FieldInfo {
        name: "avg_cadence_position",
        field_type: FieldDataType::UInt8,
        def_number: 122,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(122, field);
    // Maximum cadence by position. Data value indexes defined by rider_position_type.
    let field = FieldInfo {
        name: "max_cadence_position",
        field_type: FieldDataType::UInt8,
        def_number: 123,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(123, field);
    // total_distance / total_timer_time
    let field = FieldInfo {
        name: "enhanced_avg_speed",
        field_type: FieldDataType::UInt32,
        def_number: 124,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(124, field);
    let field = FieldInfo {
        name: "enhanced_max_speed",
        field_type: FieldDataType::UInt32,
        def_number: 125,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(125, field);
    let field = FieldInfo {
        name: "enhanced_avg_altitude",
        field_type: FieldDataType::UInt32,
        def_number: 126,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(126, field);
    let field = FieldInfo {
        name: "enhanced_min_altitude",
        field_type: FieldDataType::UInt32,
        def_number: 127,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(127, field);
    let field = FieldInfo {
        name: "enhanced_max_altitude",
        field_type: FieldDataType::UInt32,
        def_number: 128,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(128, field);
    // lev average motor power during session
    let field = FieldInfo {
        name: "avg_lev_motor_power",
        field_type: FieldDataType::UInt16,
        def_number: 129,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(129, field);
    // lev maximum motor power during session
    let field = FieldInfo {
        name: "max_lev_motor_power",
        field_type: FieldDataType::UInt16,
        def_number: 130,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(130, field);
    // lev battery consumption during session
    let field = FieldInfo {
        name: "lev_battery_consumption",
        field_type: FieldDataType::UInt8,
        def_number: 131,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(131, field);
    let field = FieldInfo {
        name: "avg_vertical_ratio",
        field_type: FieldDataType::UInt16,
        def_number: 132,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(132, field);
    let field = FieldInfo {
        name: "avg_stance_time_balance",
        field_type: FieldDataType::UInt16,
        def_number: 133,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(133, field);
    let field = FieldInfo {
        name: "avg_step_length",
        field_type: FieldDataType::UInt16,
        def_number: 134,
        scale: 10.000000,
        offset: 0.000000,
        units: "mm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(134, field);
    let field = FieldInfo {
        name: "total_anaerobic_training_effect",
        field_type: FieldDataType::UInt8,
        def_number: 137,
        scale: 10.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(137, field);
    let field = FieldInfo {
        name: "avg_vam",
        field_type: FieldDataType::UInt16,
        def_number: 139,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(139, field);
    // The grit score estimates how challenging a route could be for a cyclist in terms of time spent going over sharp turns or large grade slopes.
    let field = FieldInfo {
        name: "total_grit",
        field_type: FieldDataType::Float32,
        def_number: 181,
        scale: 1.000000,
        offset: 0.000000,
        units: "kGrit",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(181, field);
    // The flow score estimates how long distance wise a cyclist deaccelerates over intervals where deacceleration is unnecessary such as smooth turns or small grade angle intervals.
    let field = FieldInfo {
        name: "total_flow",
        field_type: FieldDataType::Float32,
        def_number: 182,
        scale: 1.000000,
        offset: 0.000000,
        units: "Flow",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(182, field);
    let field = FieldInfo {
        name: "jump_count",
        field_type: FieldDataType::UInt16,
        def_number: 183,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(183, field);
    // The grit score estimates how challenging a route could be for a cyclist in terms of time spent going over sharp turns or large grade slopes.
    let field = FieldInfo {
        name: "avg_grit",
        field_type: FieldDataType::Float32,
        def_number: 186,
        scale: 1.000000,
        offset: 0.000000,
        units: "kGrit",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(186, field);
    // The flow score estimates how long distance wise a cyclist deaccelerates over intervals where deacceleration is unnecessary such as smooth turns or small grade angle intervals.
    let field = FieldInfo {
        name: "avg_flow",
        field_type: FieldDataType::Float32,
        def_number: 187,
        scale: 1.000000,
        offset: 0.000000,
        units: "Flow",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(187, field);
    // fractional part of total_ascent
    let field = FieldInfo {
        name: "total_fractional_ascent",
        field_type: FieldDataType::UInt8,
        def_number: 199,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(199, field);
    // fractional part of total_descent
    let field = FieldInfo {
        name: "total_fractional_descent",
        field_type: FieldDataType::UInt8,
        def_number: 200,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(200, field);
    let field = FieldInfo {
        name: "avg_core_temperature",
        field_type: FieldDataType::UInt16,
        def_number: 208,
        scale: 100.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(208, field);
    let field = FieldInfo {
        name: "min_core_temperature",
        field_type: FieldDataType::UInt16,
        def_number: 209,
        scale: 100.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(209, field);
    let field = FieldInfo {
        name: "max_core_temperature",
        field_type: FieldDataType::UInt16,
        def_number: 210,
        scale: 100.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(210, field);
    // Sesson end time.
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    // Selected bit is set for the current session.
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "session",
        global_message_number: MesgNum::Session,
        fields,
    }
}
pub fn lap_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "event",
        field_type: FieldDataType::Event,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "event_type",
        field_type: FieldDataType::EventType,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "start_time",
        field_type: FieldDataType::DateTime,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "start_position_lat",
        field_type: FieldDataType::SInt32,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "start_position_long",
        field_type: FieldDataType::SInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "end_position_lat",
        field_type: FieldDataType::SInt32,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "end_position_long",
        field_type: FieldDataType::SInt32,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    // Time (includes pauses)
    let field = FieldInfo {
        name: "total_elapsed_time",
        field_type: FieldDataType::UInt32,
        def_number: 7,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    // Timer Time (excludes pauses)
    let field = FieldInfo {
        name: "total_timer_time",
        field_type: FieldDataType::UInt32,
        def_number: 8,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "total_distance",
        field_type: FieldDataType::UInt32,
        def_number: 9,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "total_strides",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "strides",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((25, Sport::Running.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "total_strides",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "strides",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((25, Sport::Walking.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "total_strokes",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "strokes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((25, Sport::Cycling.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "total_strokes",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "strokes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((25, Sport::Swimming.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "total_strokes",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "strokes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((25, Sport::Rowing.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "total_strokes",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "strokes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((25, Sport::StandUpPaddleboarding.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "total_cycles",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "cycles",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "total_calories",
        field_type: FieldDataType::UInt16,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    // If New Leaf
    let field = FieldInfo {
        name: "total_fat_calories",
        field_type: FieldDataType::UInt16,
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(12, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 110,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        bits: 16,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "avg_speed",
        field_type: FieldDataType::UInt16,
        def_number: 13,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(13, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 111,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        bits: 16,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "max_speed",
        field_type: FieldDataType::UInt16,
        def_number: 14,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(14, field);
    let field = FieldInfo {
        name: "avg_heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 15,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(15, field);
    let field = FieldInfo {
        name: "max_heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 16,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(16, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "avg_running_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 17,
        scale: 1.000000,
        offset: 0.000000,
        units: "strides/min",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((25, Sport::Running.as_i64(), sub_fld));
    // total_cycles / total_timer_time if non_zero_avg_cadence otherwise total_cycles / total_elapsed_time
    let field = FieldInfo {
        name: "avg_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 17,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(17, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "max_running_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "strides/min",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((25, Sport::Running.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "max_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(18, field);
    // total_power / total_timer_time if non_zero_avg_power otherwise total_power / total_elapsed_time
    let field = FieldInfo {
        name: "avg_power",
        field_type: FieldDataType::UInt16,
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(19, field);
    let field = FieldInfo {
        name: "max_power",
        field_type: FieldDataType::UInt16,
        def_number: 20,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(20, field);
    let field = FieldInfo {
        name: "total_ascent",
        field_type: FieldDataType::UInt16,
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(21, field);
    let field = FieldInfo {
        name: "total_descent",
        field_type: FieldDataType::UInt16,
        def_number: 22,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(22, field);
    let field = FieldInfo {
        name: "intensity",
        field_type: FieldDataType::Intensity,
        def_number: 23,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(23, field);
    let field = FieldInfo {
        name: "lap_trigger",
        field_type: FieldDataType::LapTrigger,
        def_number: 24,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(24, field);
    let field = FieldInfo {
        name: "sport",
        field_type: FieldDataType::Sport,
        def_number: 25,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(25, field);
    let field = FieldInfo {
        name: "event_group",
        field_type: FieldDataType::UInt8,
        def_number: 26,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(26, field);
    // # of lengths of swim pool
    let field = FieldInfo {
        name: "num_lengths",
        field_type: FieldDataType::UInt16,
        def_number: 32,
        scale: 1.000000,
        offset: 0.000000,
        units: "lengths",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(32, field);
    let field = FieldInfo {
        name: "normalized_power",
        field_type: FieldDataType::UInt16,
        def_number: 33,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(33, field);
    let field = FieldInfo {
        name: "left_right_balance",
        field_type: FieldDataType::LeftRightBalance100,
        def_number: 34,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(34, field);
    let field = FieldInfo {
        name: "first_length_index",
        field_type: FieldDataType::UInt16,
        def_number: 35,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(35, field);
    let field = FieldInfo {
        name: "avg_stroke_distance",
        field_type: FieldDataType::UInt16,
        def_number: 37,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(37, field);
    let field = FieldInfo {
        name: "swim_stroke",
        field_type: FieldDataType::SwimStroke,
        def_number: 38,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(38, field);
    let field = FieldInfo {
        name: "sub_sport",
        field_type: FieldDataType::SubSport,
        def_number: 39,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(39, field);
    // # of active lengths of swim pool
    let field = FieldInfo {
        name: "num_active_lengths",
        field_type: FieldDataType::UInt16,
        def_number: 40,
        scale: 1.000000,
        offset: 0.000000,
        units: "lengths",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(40, field);
    let field = FieldInfo {
        name: "total_work",
        field_type: FieldDataType::UInt32,
        def_number: 41,
        scale: 1.000000,
        offset: 0.000000,
        units: "J",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(41, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 112,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        bits: 16,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "avg_altitude",
        field_type: FieldDataType::UInt16,
        def_number: 42,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(42, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 114,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        bits: 16,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "max_altitude",
        field_type: FieldDataType::UInt16,
        def_number: 43,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(43, field);
    let field = FieldInfo {
        name: "gps_accuracy",
        field_type: FieldDataType::UInt8,
        def_number: 44,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(44, field);
    let field = FieldInfo {
        name: "avg_grade",
        field_type: FieldDataType::SInt16,
        def_number: 45,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(45, field);
    let field = FieldInfo {
        name: "avg_pos_grade",
        field_type: FieldDataType::SInt16,
        def_number: 46,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(46, field);
    let field = FieldInfo {
        name: "avg_neg_grade",
        field_type: FieldDataType::SInt16,
        def_number: 47,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(47, field);
    let field = FieldInfo {
        name: "max_pos_grade",
        field_type: FieldDataType::SInt16,
        def_number: 48,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(48, field);
    let field = FieldInfo {
        name: "max_neg_grade",
        field_type: FieldDataType::SInt16,
        def_number: 49,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(49, field);
    let field = FieldInfo {
        name: "avg_temperature",
        field_type: FieldDataType::SInt8,
        def_number: 50,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(50, field);
    let field = FieldInfo {
        name: "max_temperature",
        field_type: FieldDataType::SInt8,
        def_number: 51,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(51, field);
    let field = FieldInfo {
        name: "total_moving_time",
        field_type: FieldDataType::UInt32,
        def_number: 52,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(52, field);
    let field = FieldInfo {
        name: "avg_pos_vertical_speed",
        field_type: FieldDataType::SInt16,
        def_number: 53,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(53, field);
    let field = FieldInfo {
        name: "avg_neg_vertical_speed",
        field_type: FieldDataType::SInt16,
        def_number: 54,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(54, field);
    let field = FieldInfo {
        name: "max_pos_vertical_speed",
        field_type: FieldDataType::SInt16,
        def_number: 55,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(55, field);
    let field = FieldInfo {
        name: "max_neg_vertical_speed",
        field_type: FieldDataType::SInt16,
        def_number: 56,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(56, field);
    let field = FieldInfo {
        name: "time_in_hr_zone",
        field_type: FieldDataType::UInt32,
        def_number: 57,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(57, field);
    let field = FieldInfo {
        name: "time_in_speed_zone",
        field_type: FieldDataType::UInt32,
        def_number: 58,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(58, field);
    let field = FieldInfo {
        name: "time_in_cadence_zone",
        field_type: FieldDataType::UInt32,
        def_number: 59,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(59, field);
    let field = FieldInfo {
        name: "time_in_power_zone",
        field_type: FieldDataType::UInt32,
        def_number: 60,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(60, field);
    let field = FieldInfo {
        name: "repetition_num",
        field_type: FieldDataType::UInt16,
        def_number: 61,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(61, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 113,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        bits: 16,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "min_altitude",
        field_type: FieldDataType::UInt16,
        def_number: 62,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(62, field);
    let field = FieldInfo {
        name: "min_heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 63,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(63, field);
    let field = FieldInfo {
        name: "wkt_step_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 71,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(71, field);
    let field = FieldInfo {
        name: "opponent_score",
        field_type: FieldDataType::UInt16,
        def_number: 74,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(74, field);
    // stroke_type enum used as the index
    let field = FieldInfo {
        name: "stroke_count",
        field_type: FieldDataType::UInt16,
        def_number: 75,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(75, field);
    // zone number used as the index
    let field = FieldInfo {
        name: "zone_count",
        field_type: FieldDataType::UInt16,
        def_number: 76,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(76, field);
    let field = FieldInfo {
        name: "avg_vertical_oscillation",
        field_type: FieldDataType::UInt16,
        def_number: 77,
        scale: 10.000000,
        offset: 0.000000,
        units: "mm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(77, field);
    let field = FieldInfo {
        name: "avg_stance_time_percent",
        field_type: FieldDataType::UInt16,
        def_number: 78,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(78, field);
    let field = FieldInfo {
        name: "avg_stance_time",
        field_type: FieldDataType::UInt16,
        def_number: 79,
        scale: 10.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(79, field);
    // fractional part of the avg_cadence
    let field = FieldInfo {
        name: "avg_fractional_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 80,
        scale: 128.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(80, field);
    // fractional part of the max_cadence
    let field = FieldInfo {
        name: "max_fractional_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 81,
        scale: 128.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(81, field);
    // fractional part of the total_cycles
    let field = FieldInfo {
        name: "total_fractional_cycles",
        field_type: FieldDataType::UInt8,
        def_number: 82,
        scale: 128.000000,
        offset: 0.000000,
        units: "cycles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(82, field);
    let field = FieldInfo {
        name: "player_score",
        field_type: FieldDataType::UInt16,
        def_number: 83,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(83, field);
    // Avg saturated and unsaturated hemoglobin
    let field = FieldInfo {
        name: "avg_total_hemoglobin_conc",
        field_type: FieldDataType::UInt16,
        def_number: 84,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(84, field);
    // Min saturated and unsaturated hemoglobin
    let field = FieldInfo {
        name: "min_total_hemoglobin_conc",
        field_type: FieldDataType::UInt16,
        def_number: 85,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(85, field);
    // Max saturated and unsaturated hemoglobin
    let field = FieldInfo {
        name: "max_total_hemoglobin_conc",
        field_type: FieldDataType::UInt16,
        def_number: 86,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(86, field);
    // Avg percentage of hemoglobin saturated with oxygen
    let field = FieldInfo {
        name: "avg_saturated_hemoglobin_percent",
        field_type: FieldDataType::UInt16,
        def_number: 87,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(87, field);
    // Min percentage of hemoglobin saturated with oxygen
    let field = FieldInfo {
        name: "min_saturated_hemoglobin_percent",
        field_type: FieldDataType::UInt16,
        def_number: 88,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(88, field);
    // Max percentage of hemoglobin saturated with oxygen
    let field = FieldInfo {
        name: "max_saturated_hemoglobin_percent",
        field_type: FieldDataType::UInt16,
        def_number: 89,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(89, field);
    let field = FieldInfo {
        name: "avg_left_torque_effectiveness",
        field_type: FieldDataType::UInt8,
        def_number: 91,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(91, field);
    let field = FieldInfo {
        name: "avg_right_torque_effectiveness",
        field_type: FieldDataType::UInt8,
        def_number: 92,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(92, field);
    let field = FieldInfo {
        name: "avg_left_pedal_smoothness",
        field_type: FieldDataType::UInt8,
        def_number: 93,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(93, field);
    let field = FieldInfo {
        name: "avg_right_pedal_smoothness",
        field_type: FieldDataType::UInt8,
        def_number: 94,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(94, field);
    let field = FieldInfo {
        name: "avg_combined_pedal_smoothness",
        field_type: FieldDataType::UInt8,
        def_number: 95,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(95, field);
    // Total time spent in the standing position
    let field = FieldInfo {
        name: "time_standing",
        field_type: FieldDataType::UInt32,
        def_number: 98,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(98, field);
    // Number of transitions to the standing state
    let field = FieldInfo {
        name: "stand_count",
        field_type: FieldDataType::UInt16,
        def_number: 99,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(99, field);
    // Average left platform center offset
    let field = FieldInfo {
        name: "avg_left_pco",
        field_type: FieldDataType::SInt8,
        def_number: 100,
        scale: 1.000000,
        offset: 0.000000,
        units: "mm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(100, field);
    // Average right platform center offset
    let field = FieldInfo {
        name: "avg_right_pco",
        field_type: FieldDataType::SInt8,
        def_number: 101,
        scale: 1.000000,
        offset: 0.000000,
        units: "mm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(101, field);
    // Average left power phase angles. Data value indexes defined by power_phase_type.
    let field = FieldInfo {
        name: "avg_left_power_phase",
        field_type: FieldDataType::UInt8,
        def_number: 102,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(102, field);
    // Average left power phase peak angles. Data value indexes  defined by power_phase_type.
    let field = FieldInfo {
        name: "avg_left_power_phase_peak",
        field_type: FieldDataType::UInt8,
        def_number: 103,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(103, field);
    // Average right power phase angles. Data value indexes defined by power_phase_type.
    let field = FieldInfo {
        name: "avg_right_power_phase",
        field_type: FieldDataType::UInt8,
        def_number: 104,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(104, field);
    // Average right power phase peak angles. Data value indexes  defined by power_phase_type.
    let field = FieldInfo {
        name: "avg_right_power_phase_peak",
        field_type: FieldDataType::UInt8,
        def_number: 105,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(105, field);
    // Average power by position. Data value indexes defined by rider_position_type.
    let field = FieldInfo {
        name: "avg_power_position",
        field_type: FieldDataType::UInt16,
        def_number: 106,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(106, field);
    // Maximum power by position. Data value indexes defined by rider_position_type.
    let field = FieldInfo {
        name: "max_power_position",
        field_type: FieldDataType::UInt16,
        def_number: 107,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(107, field);
    // Average cadence by position. Data value indexes defined by rider_position_type.
    let field = FieldInfo {
        name: "avg_cadence_position",
        field_type: FieldDataType::UInt8,
        def_number: 108,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(108, field);
    // Maximum cadence by position. Data value indexes defined by rider_position_type.
    let field = FieldInfo {
        name: "max_cadence_position",
        field_type: FieldDataType::UInt8,
        def_number: 109,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(109, field);
    let field = FieldInfo {
        name: "enhanced_avg_speed",
        field_type: FieldDataType::UInt32,
        def_number: 110,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(110, field);
    let field = FieldInfo {
        name: "enhanced_max_speed",
        field_type: FieldDataType::UInt32,
        def_number: 111,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(111, field);
    let field = FieldInfo {
        name: "enhanced_avg_altitude",
        field_type: FieldDataType::UInt32,
        def_number: 112,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(112, field);
    let field = FieldInfo {
        name: "enhanced_min_altitude",
        field_type: FieldDataType::UInt32,
        def_number: 113,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(113, field);
    let field = FieldInfo {
        name: "enhanced_max_altitude",
        field_type: FieldDataType::UInt32,
        def_number: 114,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(114, field);
    // lev average motor power during lap
    let field = FieldInfo {
        name: "avg_lev_motor_power",
        field_type: FieldDataType::UInt16,
        def_number: 115,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(115, field);
    // lev maximum motor power during lap
    let field = FieldInfo {
        name: "max_lev_motor_power",
        field_type: FieldDataType::UInt16,
        def_number: 116,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(116, field);
    // lev battery consumption during lap
    let field = FieldInfo {
        name: "lev_battery_consumption",
        field_type: FieldDataType::UInt8,
        def_number: 117,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(117, field);
    let field = FieldInfo {
        name: "avg_vertical_ratio",
        field_type: FieldDataType::UInt16,
        def_number: 118,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(118, field);
    let field = FieldInfo {
        name: "avg_stance_time_balance",
        field_type: FieldDataType::UInt16,
        def_number: 119,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(119, field);
    let field = FieldInfo {
        name: "avg_step_length",
        field_type: FieldDataType::UInt16,
        def_number: 120,
        scale: 10.000000,
        offset: 0.000000,
        units: "mm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(120, field);
    let field = FieldInfo {
        name: "avg_vam",
        field_type: FieldDataType::UInt16,
        def_number: 121,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(121, field);
    // The grit score estimates how challenging a route could be for a cyclist in terms of time spent going over sharp turns or large grade slopes.
    let field = FieldInfo {
        name: "total_grit",
        field_type: FieldDataType::Float32,
        def_number: 149,
        scale: 1.000000,
        offset: 0.000000,
        units: "kGrit",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(149, field);
    // The flow score estimates how long distance wise a cyclist deaccelerates over intervals where deacceleration is unnecessary such as smooth turns or small grade angle intervals.
    let field = FieldInfo {
        name: "total_flow",
        field_type: FieldDataType::Float32,
        def_number: 150,
        scale: 1.000000,
        offset: 0.000000,
        units: "Flow",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(150, field);
    let field = FieldInfo {
        name: "jump_count",
        field_type: FieldDataType::UInt16,
        def_number: 151,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(151, field);
    // The grit score estimates how challenging a route could be for a cyclist in terms of time spent going over sharp turns or large grade slopes.
    let field = FieldInfo {
        name: "avg_grit",
        field_type: FieldDataType::Float32,
        def_number: 153,
        scale: 1.000000,
        offset: 0.000000,
        units: "kGrit",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(153, field);
    // The flow score estimates how long distance wise a cyclist deaccelerates over intervals where deacceleration is unnecessary such as smooth turns or small grade angle intervals.
    let field = FieldInfo {
        name: "avg_flow",
        field_type: FieldDataType::Float32,
        def_number: 154,
        scale: 1.000000,
        offset: 0.000000,
        units: "Flow",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(154, field);
    // fractional part of total_ascent
    let field = FieldInfo {
        name: "total_fractional_ascent",
        field_type: FieldDataType::UInt8,
        def_number: 156,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(156, field);
    // fractional part of total_descent
    let field = FieldInfo {
        name: "total_fractional_descent",
        field_type: FieldDataType::UInt8,
        def_number: 157,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(157, field);
    let field = FieldInfo {
        name: "avg_core_temperature",
        field_type: FieldDataType::UInt16,
        def_number: 158,
        scale: 100.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(158, field);
    let field = FieldInfo {
        name: "min_core_temperature",
        field_type: FieldDataType::UInt16,
        def_number: 159,
        scale: 100.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(159, field);
    let field = FieldInfo {
        name: "max_core_temperature",
        field_type: FieldDataType::UInt16,
        def_number: 160,
        scale: 100.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(160, field);
    // Lap end time.
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "lap",
        global_message_number: MesgNum::Lap,
        fields,
    }
}
pub fn length_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "event",
        field_type: FieldDataType::Event,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "event_type",
        field_type: FieldDataType::EventType,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "start_time",
        field_type: FieldDataType::DateTime,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "total_elapsed_time",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "total_timer_time",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "total_strokes",
        field_type: FieldDataType::UInt16,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "strokes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "avg_speed",
        field_type: FieldDataType::UInt16,
        def_number: 6,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "swim_stroke",
        field_type: FieldDataType::SwimStroke,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "swim_stroke",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    let field = FieldInfo {
        name: "avg_swimming_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "strokes/min",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "event_group",
        field_type: FieldDataType::UInt8,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "total_calories",
        field_type: FieldDataType::UInt16,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    let field = FieldInfo {
        name: "length_type",
        field_type: FieldDataType::LengthType,
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(12, field);
    let field = FieldInfo {
        name: "player_score",
        field_type: FieldDataType::UInt16,
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(18, field);
    let field = FieldInfo {
        name: "opponent_score",
        field_type: FieldDataType::UInt16,
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(19, field);
    // stroke_type enum used as the index
    let field = FieldInfo {
        name: "stroke_count",
        field_type: FieldDataType::UInt16,
        def_number: 20,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(20, field);
    // zone number used as the index
    let field = FieldInfo {
        name: "zone_count",
        field_type: FieldDataType::UInt16,
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(21, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "length",
        global_message_number: MesgNum::Length,
        fields,
    }
}
pub fn record_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "position_lat",
        field_type: FieldDataType::SInt32,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "position_long",
        field_type: FieldDataType::SInt32,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 78,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        bits: 16,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "altitude",
        field_type: FieldDataType::UInt16,
        def_number: 2,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "cadence",
        field_type: FieldDataType::UInt8,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "distance",
        field_type: FieldDataType::UInt32,
        def_number: 5,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 73,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        bits: 16,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "speed",
        field_type: FieldDataType::UInt16,
        def_number: 6,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "power",
        field_type: FieldDataType::UInt16,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 6,
        scale: 100.000000,
        offset: 0.000000,
        units: "m/s",
        bits: 12,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 5,
        scale: 16.000000,
        offset: 0.000000,
        units: "m",
        bits: 12,
        accumulate: true,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "compressed_speed_distance",
        field_type: FieldDataType::Byte,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "m/s,
m",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "grade",
        field_type: FieldDataType::SInt16,
        def_number: 9,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    // Relative. 0 is none  254 is Max.
    let field = FieldInfo {
        name: "resistance",
        field_type: FieldDataType::UInt8,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "time_from_course",
        field_type: FieldDataType::SInt32,
        def_number: 11,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    let field = FieldInfo {
        name: "cycle_length",
        field_type: FieldDataType::UInt8,
        def_number: 12,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(12, field);
    let field = FieldInfo {
        name: "temperature",
        field_type: FieldDataType::SInt8,
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(13, field);
    // Speed at 1s intervals.  Timestamp field indicates time of last array element.
    let field = FieldInfo {
        name: "speed_1s",
        field_type: FieldDataType::UInt8,
        def_number: 17,
        scale: 16.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(17, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "cycles",
        bits: 8,
        accumulate: true,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "cycles",
        field_type: FieldDataType::UInt8,
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "cycles",
        accumulate: true,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(18, field);
    let field = FieldInfo {
        name: "total_cycles",
        field_type: FieldDataType::UInt32,
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "cycles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(19, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 29,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        bits: 16,
        accumulate: true,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "compressed_accumulated_power",
        field_type: FieldDataType::UInt16,
        def_number: 28,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: true,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(28, field);
    let field = FieldInfo {
        name: "accumulated_power",
        field_type: FieldDataType::UInt32,
        def_number: 29,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(29, field);
    let field = FieldInfo {
        name: "left_right_balance",
        field_type: FieldDataType::LeftRightBalance,
        def_number: 30,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(30, field);
    let field = FieldInfo {
        name: "gps_accuracy",
        field_type: FieldDataType::UInt8,
        def_number: 31,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(31, field);
    let field = FieldInfo {
        name: "vertical_speed",
        field_type: FieldDataType::SInt16,
        def_number: 32,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(32, field);
    let field = FieldInfo {
        name: "calories",
        field_type: FieldDataType::UInt16,
        def_number: 33,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(33, field);
    let field = FieldInfo {
        name: "vertical_oscillation",
        field_type: FieldDataType::UInt16,
        def_number: 39,
        scale: 10.000000,
        offset: 0.000000,
        units: "mm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(39, field);
    let field = FieldInfo {
        name: "stance_time_percent",
        field_type: FieldDataType::UInt16,
        def_number: 40,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(40, field);
    let field = FieldInfo {
        name: "stance_time",
        field_type: FieldDataType::UInt16,
        def_number: 41,
        scale: 10.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(41, field);
    let field = FieldInfo {
        name: "activity_type",
        field_type: FieldDataType::ActivityType,
        def_number: 42,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(42, field);
    let field = FieldInfo {
        name: "left_torque_effectiveness",
        field_type: FieldDataType::UInt8,
        def_number: 43,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(43, field);
    let field = FieldInfo {
        name: "right_torque_effectiveness",
        field_type: FieldDataType::UInt8,
        def_number: 44,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(44, field);
    let field = FieldInfo {
        name: "left_pedal_smoothness",
        field_type: FieldDataType::UInt8,
        def_number: 45,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(45, field);
    let field = FieldInfo {
        name: "right_pedal_smoothness",
        field_type: FieldDataType::UInt8,
        def_number: 46,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(46, field);
    let field = FieldInfo {
        name: "combined_pedal_smoothness",
        field_type: FieldDataType::UInt8,
        def_number: 47,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(47, field);
    let field = FieldInfo {
        name: "time128",
        field_type: FieldDataType::UInt8,
        def_number: 48,
        scale: 128.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(48, field);
    let field = FieldInfo {
        name: "stroke_type",
        field_type: FieldDataType::StrokeType,
        def_number: 49,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(49, field);
    let field = FieldInfo {
        name: "zone",
        field_type: FieldDataType::UInt8,
        def_number: 50,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(50, field);
    let field = FieldInfo {
        name: "ball_speed",
        field_type: FieldDataType::UInt16,
        def_number: 51,
        scale: 100.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(51, field);
    // Log cadence and fractional cadence for backwards compatability
    let field = FieldInfo {
        name: "cadence256",
        field_type: FieldDataType::UInt16,
        def_number: 52,
        scale: 256.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(52, field);
    let field = FieldInfo {
        name: "fractional_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 53,
        scale: 128.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(53, field);
    // Total saturated and unsaturated hemoglobin
    let field = FieldInfo {
        name: "total_hemoglobin_conc",
        field_type: FieldDataType::UInt16,
        def_number: 54,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(54, field);
    // Min saturated and unsaturated hemoglobin
    let field = FieldInfo {
        name: "total_hemoglobin_conc_min",
        field_type: FieldDataType::UInt16,
        def_number: 55,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(55, field);
    // Max saturated and unsaturated hemoglobin
    let field = FieldInfo {
        name: "total_hemoglobin_conc_max",
        field_type: FieldDataType::UInt16,
        def_number: 56,
        scale: 100.000000,
        offset: 0.000000,
        units: "g/dL",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(56, field);
    // Percentage of hemoglobin saturated with oxygen
    let field = FieldInfo {
        name: "saturated_hemoglobin_percent",
        field_type: FieldDataType::UInt16,
        def_number: 57,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(57, field);
    // Min percentage of hemoglobin saturated with oxygen
    let field = FieldInfo {
        name: "saturated_hemoglobin_percent_min",
        field_type: FieldDataType::UInt16,
        def_number: 58,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(58, field);
    // Max percentage of hemoglobin saturated with oxygen
    let field = FieldInfo {
        name: "saturated_hemoglobin_percent_max",
        field_type: FieldDataType::UInt16,
        def_number: 59,
        scale: 10.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(59, field);
    let field = FieldInfo {
        name: "device_index",
        field_type: FieldDataType::DeviceIndex,
        def_number: 62,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(62, field);
    // Left platform center offset
    let field = FieldInfo {
        name: "left_pco",
        field_type: FieldDataType::SInt8,
        def_number: 67,
        scale: 1.000000,
        offset: 0.000000,
        units: "mm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(67, field);
    // Right platform center offset
    let field = FieldInfo {
        name: "right_pco",
        field_type: FieldDataType::SInt8,
        def_number: 68,
        scale: 1.000000,
        offset: 0.000000,
        units: "mm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(68, field);
    // Left power phase angles. Data value indexes defined by power_phase_type.
    let field = FieldInfo {
        name: "left_power_phase",
        field_type: FieldDataType::UInt8,
        def_number: 69,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(69, field);
    // Left power phase peak angles. Data value indexes defined by power_phase_type.
    let field = FieldInfo {
        name: "left_power_phase_peak",
        field_type: FieldDataType::UInt8,
        def_number: 70,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(70, field);
    // Right power phase angles. Data value indexes defined by power_phase_type.
    let field = FieldInfo {
        name: "right_power_phase",
        field_type: FieldDataType::UInt8,
        def_number: 71,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(71, field);
    // Right power phase peak angles. Data value indexes defined by power_phase_type.
    let field = FieldInfo {
        name: "right_power_phase_peak",
        field_type: FieldDataType::UInt8,
        def_number: 72,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(72, field);
    let field = FieldInfo {
        name: "enhanced_speed",
        field_type: FieldDataType::UInt32,
        def_number: 73,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(73, field);
    let field = FieldInfo {
        name: "enhanced_altitude",
        field_type: FieldDataType::UInt32,
        def_number: 78,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(78, field);
    // lev battery state of charge
    let field = FieldInfo {
        name: "battery_soc",
        field_type: FieldDataType::UInt8,
        def_number: 81,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(81, field);
    // lev motor power
    let field = FieldInfo {
        name: "motor_power",
        field_type: FieldDataType::UInt16,
        def_number: 82,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(82, field);
    let field = FieldInfo {
        name: "vertical_ratio",
        field_type: FieldDataType::UInt16,
        def_number: 83,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(83, field);
    let field = FieldInfo {
        name: "stance_time_balance",
        field_type: FieldDataType::UInt16,
        def_number: 84,
        scale: 100.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(84, field);
    let field = FieldInfo {
        name: "step_length",
        field_type: FieldDataType::UInt16,
        def_number: 85,
        scale: 10.000000,
        offset: 0.000000,
        units: "mm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(85, field);
    // Includes atmospheric pressure
    let field = FieldInfo {
        name: "absolute_pressure",
        field_type: FieldDataType::UInt32,
        def_number: 91,
        scale: 1.000000,
        offset: 0.000000,
        units: "Pa",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(91, field);
    // 0 if above water
    let field = FieldInfo {
        name: "depth",
        field_type: FieldDataType::UInt32,
        def_number: 92,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(92, field);
    // 0 if above water
    let field = FieldInfo {
        name: "next_stop_depth",
        field_type: FieldDataType::UInt32,
        def_number: 93,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(93, field);
    let field = FieldInfo {
        name: "next_stop_time",
        field_type: FieldDataType::UInt32,
        def_number: 94,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(94, field);
    let field = FieldInfo {
        name: "time_to_surface",
        field_type: FieldDataType::UInt32,
        def_number: 95,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(95, field);
    let field = FieldInfo {
        name: "ndl_time",
        field_type: FieldDataType::UInt32,
        def_number: 96,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(96, field);
    let field = FieldInfo {
        name: "cns_load",
        field_type: FieldDataType::UInt8,
        def_number: 97,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(97, field);
    let field = FieldInfo {
        name: "n2_load",
        field_type: FieldDataType::UInt16,
        def_number: 98,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(98, field);
    // The grit score estimates how challenging a route could be for a cyclist in terms of time spent going over sharp turns or large grade slopes.
    let field = FieldInfo {
        name: "grit",
        field_type: FieldDataType::Float32,
        def_number: 114,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(114, field);
    // The flow score estimates how long distance wise a cyclist deaccelerates over intervals where deacceleration is unnecessary such as smooth turns or small grade angle intervals.
    let field = FieldInfo {
        name: "flow",
        field_type: FieldDataType::Float32,
        def_number: 115,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(115, field);
    let field = FieldInfo {
        name: "ebike_travel_range",
        field_type: FieldDataType::UInt16,
        def_number: 117,
        scale: 1.000000,
        offset: 0.000000,
        units: "km",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(117, field);
    let field = FieldInfo {
        name: "ebike_battery_level",
        field_type: FieldDataType::UInt8,
        def_number: 118,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(118, field);
    let field = FieldInfo {
        name: "ebike_assist_mode",
        field_type: FieldDataType::UInt8,
        def_number: 119,
        scale: 1.000000,
        offset: 0.000000,
        units: "depends on sensor",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(119, field);
    let field = FieldInfo {
        name: "ebike_assist_level_percent",
        field_type: FieldDataType::UInt8,
        def_number: 120,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(120, field);
    let field = FieldInfo {
        name: "core_temperature",
        field_type: FieldDataType::UInt16,
        def_number: 139,
        scale: 100.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(139, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "record",
        global_message_number: MesgNum::Record,
        fields,
    }
}
pub fn event_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "event",
        field_type: FieldDataType::Event,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "event_type",
        field_type: FieldDataType::EventType,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 16,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "data16",
        field_type: FieldDataType::UInt16,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(2, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "timer_trigger",
        field_type: FieldDataType::TimerTrigger,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::Timer.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "course_point_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::CoursePoint.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "battery_level",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1000.000000,
        offset: 0.000000,
        units: "V",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::Battery.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "virtual_partner_speed",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::VirtualPartnerPace.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "hr_high_alert",
        field_type: FieldDataType::UInt8,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::HrHighAlert.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "hr_low_alert",
        field_type: FieldDataType::UInt8,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::HrLowAlert.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "speed_high_alert",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::SpeedHighAlert.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "speed_low_alert",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::SpeedLowAlert.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "cad_high_alert",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::CadHighAlert.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "cad_low_alert",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::CadLowAlert.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "power_high_alert",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::PowerHighAlert.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "power_low_alert",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::PowerLowAlert.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "time_duration_alert",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::TimeDurationAlert.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "distance_duration_alert",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::DistanceDurationAlert.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "calorie_duration_alert",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "calories",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::CalorieDurationAlert.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "fitness_equipment_state",
        field_type: FieldDataType::FitnessEquipmentState,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::FitnessEquipment.as_i64(), sub_fld));
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 16,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 16,
        accumulate: false,
    };
    components.push(comp_fld);
    let sub_fld = FieldInfo {
        name: "sport_point",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    subfields.push((0, Event::SportPoint.as_i64(), sub_fld));
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let sub_fld = FieldInfo {
        name: "gear_change_data",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    subfields.push((0, Event::FrontGearChange.as_i64(), sub_fld));
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let sub_fld = FieldInfo {
        name: "gear_change_data",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    subfields.push((0, Event::RearGearChange.as_i64(), sub_fld));
    // Indicates the rider position value.
    let sub_fld = FieldInfo {
        name: "rider_position",
        field_type: FieldDataType::RiderPositionType,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::RiderPositionChange.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "comm_timeout",
        field_type: FieldDataType::CommTimeoutType,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Event::CommTimeout.as_i64(), sub_fld));
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 22,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    // The first byte is the radar_threat_level_max, the second byte is the radar_threat_count, and the last 16 bits are reserved for future use and should be set to FFFF.
    let sub_fld = FieldInfo {
        name: "radar_threat_alert",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    subfields.push((0, Event::RadarThreatAlert.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "data",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "event_group",
        field_type: FieldDataType::UInt8,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // Do not populate directly. Autogenerated by decoder for sport_point subfield components
    let field = FieldInfo {
        name: "score",
        field_type: FieldDataType::UInt16,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    // Do not populate directly. Autogenerated by decoder for sport_point subfield components
    let field = FieldInfo {
        name: "opponent_score",
        field_type: FieldDataType::UInt16,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    // Do not populate directly. Autogenerated by decoder for gear_change subfield components.  Front gear number. 1 is innermost.
    let field = FieldInfo {
        name: "front_gear_num",
        field_type: FieldDataType::UInt8z,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    // Do not populate directly. Autogenerated by decoder for gear_change subfield components.  Number of front teeth.
    let field = FieldInfo {
        name: "front_gear",
        field_type: FieldDataType::UInt8z,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    // Do not populate directly. Autogenerated by decoder for gear_change subfield components.  Rear gear number. 1 is innermost.
    let field = FieldInfo {
        name: "rear_gear_num",
        field_type: FieldDataType::UInt8z,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    // Do not populate directly. Autogenerated by decoder for gear_change subfield components.  Number of rear teeth.
    let field = FieldInfo {
        name: "rear_gear",
        field_type: FieldDataType::UInt8z,
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(12, field);
    let field = FieldInfo {
        name: "device_index",
        field_type: FieldDataType::DeviceIndex,
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(13, field);
    // Do not populate directly. Autogenerated by decoder for threat_alert subfield components.
    let field = FieldInfo {
        name: "radar_threat_level_max",
        field_type: FieldDataType::RadarThreatLevelType,
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(21, field);
    // Do not populate directly. Autogenerated by decoder for threat_alert subfield components.
    let field = FieldInfo {
        name: "radar_threat_count",
        field_type: FieldDataType::UInt8,
        def_number: 22,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(22, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "event",
        global_message_number: MesgNum::Event,
        fields,
    }
}
pub fn device_info_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "device_index",
        field_type: FieldDataType::DeviceIndex,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "antplus_device_type",
        field_type: FieldDataType::AntplusDeviceType,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((25, SourceType::Antplus.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "ant_device_type",
        field_type: FieldDataType::UInt8,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((25, SourceType::Ant.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "device_type",
        field_type: FieldDataType::UInt8,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "manufacturer",
        field_type: FieldDataType::Manufacturer,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "serial_number",
        field_type: FieldDataType::UInt32z,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "favero_product",
        field_type: FieldDataType::FaveroProduct,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((2, Manufacturer::FaveroElectronics.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((2, Manufacturer::Garmin.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((2, Manufacturer::Dynastream.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((2, Manufacturer::DynastreamOem.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((2, Manufacturer::Tacx.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "product",
        field_type: FieldDataType::UInt16,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "software_version",
        field_type: FieldDataType::UInt16,
        def_number: 5,
        scale: 100.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "hardware_version",
        field_type: FieldDataType::UInt8,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    // Reset by new battery or charge.
    let field = FieldInfo {
        name: "cum_operating_time",
        field_type: FieldDataType::UInt32,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    let field = FieldInfo {
        name: "battery_voltage",
        field_type: FieldDataType::UInt16,
        def_number: 10,
        scale: 256.000000,
        offset: 0.000000,
        units: "V",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "battery_status",
        field_type: FieldDataType::BatteryStatus,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    // Indicates the location of the sensor
    let field = FieldInfo {
        name: "sensor_position",
        field_type: FieldDataType::BodyLocation,
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(18, field);
    // Used to describe the sensor or location
    let field = FieldInfo {
        name: "descriptor",
        field_type: FieldDataType::String,
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(19, field);
    let field = FieldInfo {
        name: "ant_transmission_type",
        field_type: FieldDataType::UInt8z,
        def_number: 20,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(20, field);
    let field = FieldInfo {
        name: "ant_device_number",
        field_type: FieldDataType::UInt16z,
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(21, field);
    let field = FieldInfo {
        name: "ant_network",
        field_type: FieldDataType::AntNetwork,
        def_number: 22,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(22, field);
    let field = FieldInfo {
        name: "source_type",
        field_type: FieldDataType::SourceType,
        def_number: 25,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(25, field);
    // Optional free form string to indicate the devices name or model
    let field = FieldInfo {
        name: "product_name",
        field_type: FieldDataType::String,
        def_number: 27,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(27, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "device_info",
        global_message_number: MesgNum::DeviceInfo,
        fields,
    }
}
/// Corresponds to file_id of workout or course.
pub fn training_file_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "type",
        field_type: FieldDataType::File,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "manufacturer",
        field_type: FieldDataType::Manufacturer,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "favero_product",
        field_type: FieldDataType::FaveroProduct,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, Manufacturer::FaveroElectronics.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, Manufacturer::Garmin.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, Manufacturer::Dynastream.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, Manufacturer::DynastreamOem.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, Manufacturer::Tacx.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "product",
        field_type: FieldDataType::UInt16,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "serial_number",
        field_type: FieldDataType::UInt32z,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "time_created",
        field_type: FieldDataType::DateTime,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "training_file",
        global_message_number: MesgNum::TrainingFile,
        fields,
    }
}
/// Heart rate variability
pub fn hrv_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Time between beats
    let field = FieldInfo {
        name: "time",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    MessageInfo {
        name: "hrv",
        global_message_number: MesgNum::Hrv,
        fields,
    }
}
pub fn weather_conditions_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Current or forecast
    let field = FieldInfo {
        name: "weather_report",
        field_type: FieldDataType::WeatherReport,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "temperature",
        field_type: FieldDataType::SInt8,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Corresponds to GSC Response weatherIcon field
    let field = FieldInfo {
        name: "condition",
        field_type: FieldDataType::WeatherStatus,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "wind_direction",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "wind_speed",
        field_type: FieldDataType::UInt16,
        def_number: 4,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // range 0-100
    let field = FieldInfo {
        name: "precipitation_probability",
        field_type: FieldDataType::UInt8,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // Heat Index if  GCS heatIdx above or equal to 90F or wind chill if GCS windChill below or equal to 32F
    let field = FieldInfo {
        name: "temperature_feels_like",
        field_type: FieldDataType::SInt8,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "relative_humidity",
        field_type: FieldDataType::UInt8,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    // string corresponding to GCS response location string
    let field = FieldInfo {
        name: "location",
        field_type: FieldDataType::String,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "observed_at_time",
        field_type: FieldDataType::DateTime,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "observed_location_lat",
        field_type: FieldDataType::SInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "observed_location_long",
        field_type: FieldDataType::SInt32,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    let field = FieldInfo {
        name: "day_of_week",
        field_type: FieldDataType::DayOfWeek,
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(12, field);
    let field = FieldInfo {
        name: "high_temperature",
        field_type: FieldDataType::SInt8,
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(13, field);
    let field = FieldInfo {
        name: "low_temperature",
        field_type: FieldDataType::SInt8,
        def_number: 14,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(14, field);
    // time of update for current conditions, else forecast time
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "weather_conditions",
        global_message_number: MesgNum::WeatherConditions,
        fields,
    }
}
pub fn weather_alert_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Unique identifier from GCS report ID string, length is 12
    let field = FieldInfo {
        name: "report_id",
        field_type: FieldDataType::String,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // Time alert was issued
    let field = FieldInfo {
        name: "issue_time",
        field_type: FieldDataType::DateTime,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Time alert expires
    let field = FieldInfo {
        name: "expire_time",
        field_type: FieldDataType::DateTime,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // Warning, Watch, Advisory, Statement
    let field = FieldInfo {
        name: "severity",
        field_type: FieldDataType::WeatherSeverity,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Tornado, Severe Thunderstorm, etc.
    let field = FieldInfo {
        name: "type",
        field_type: FieldDataType::WeatherSevereType,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "weather_alert",
        global_message_number: MesgNum::WeatherAlert,
        fields,
    }
}
pub fn gps_metadata_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Millisecond part of the timestamp.
    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "position_lat",
        field_type: FieldDataType::SInt32,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "position_long",
        field_type: FieldDataType::SInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "enhanced_altitude",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "enhanced_speed",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "heading",
        field_type: FieldDataType::UInt16,
        def_number: 5,
        scale: 100.000000,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // Used to correlate UTC to system time if the timestamp of the message is in system time.  This UTC time is derived from the GPS data.
    let field = FieldInfo {
        name: "utc_timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    // velocity[0] is lon velocity.  Velocity[1] is lat velocity.  Velocity[2] is altitude velocity.
    let field = FieldInfo {
        name: "velocity",
        field_type: FieldDataType::SInt16,
        def_number: 7,
        scale: 100.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    // Whole second part of the timestamp.
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "gps_metadata",
        global_message_number: MesgNum::GpsMetadata,
        fields,
    }
}
pub fn camera_event_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Millisecond part of the timestamp.
    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "camera_event_type",
        field_type: FieldDataType::CameraEventType,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "camera_file_uuid",
        field_type: FieldDataType::String,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "camera_orientation",
        field_type: FieldDataType::CameraOrientationType,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Whole second part of the timestamp.
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "camera_event",
        global_message_number: MesgNum::CameraEvent,
        fields,
    }
}
pub fn gyroscope_data_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Millisecond part of the timestamp.
    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // Each time in the array describes the time at which the gyro sample with the corrosponding index was taken. Limited to 30 samples in each message. The samples may span across seconds. Array size must match the number of samples in gyro_x and gyro_y and gyro_z
    let field = FieldInfo {
        name: "sample_time_offset",
        field_type: FieldDataType::UInt16,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // These are the raw ADC reading. Maximum number of samples is 30 in each message. The samples may span across seconds. A conversion will need to be done on this data once read.
    let field = FieldInfo {
        name: "gyro_x",
        field_type: FieldDataType::UInt16,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // These are the raw ADC reading. Maximum number of samples is 30 in each message. The samples may span across seconds. A conversion will need to be done on this data once read.
    let field = FieldInfo {
        name: "gyro_y",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // These are the raw ADC reading. Maximum number of samples is 30 in each message. The samples may span across seconds. A conversion will need to be done on this data once read.
    let field = FieldInfo {
        name: "gyro_z",
        field_type: FieldDataType::UInt16,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // Calibrated gyro reading
    let field = FieldInfo {
        name: "calibrated_gyro_x",
        field_type: FieldDataType::Float32,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "deg/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // Calibrated gyro reading
    let field = FieldInfo {
        name: "calibrated_gyro_y",
        field_type: FieldDataType::Float32,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "deg/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    // Calibrated gyro reading
    let field = FieldInfo {
        name: "calibrated_gyro_z",
        field_type: FieldDataType::Float32,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "deg/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    // Whole second part of the timestamp
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "gyroscope_data",
        global_message_number: MesgNum::GyroscopeData,
        fields,
    }
}
pub fn accelerometer_data_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Millisecond part of the timestamp.
    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // Each time in the array describes the time at which the accelerometer sample with the corrosponding index was taken. Limited to 30 samples in each message. The samples may span across seconds. Array size must match the number of samples in accel_x and accel_y and accel_z
    let field = FieldInfo {
        name: "sample_time_offset",
        field_type: FieldDataType::UInt16,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // These are the raw ADC reading. Maximum number of samples is 30 in each message. The samples may span across seconds. A conversion will need to be done on this data once read.
    let field = FieldInfo {
        name: "accel_x",
        field_type: FieldDataType::UInt16,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // These are the raw ADC reading. Maximum number of samples is 30 in each message. The samples may span across seconds. A conversion will need to be done on this data once read.
    let field = FieldInfo {
        name: "accel_y",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // These are the raw ADC reading. Maximum number of samples is 30 in each message. The samples may span across seconds. A conversion will need to be done on this data once read.
    let field = FieldInfo {
        name: "accel_z",
        field_type: FieldDataType::UInt16,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // Calibrated accel reading
    let field = FieldInfo {
        name: "calibrated_accel_x",
        field_type: FieldDataType::Float32,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "g",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // Calibrated accel reading
    let field = FieldInfo {
        name: "calibrated_accel_y",
        field_type: FieldDataType::Float32,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "g",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    // Calibrated accel reading
    let field = FieldInfo {
        name: "calibrated_accel_z",
        field_type: FieldDataType::Float32,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "g",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    // Calibrated accel reading
    let field = FieldInfo {
        name: "compressed_calibrated_accel_x",
        field_type: FieldDataType::SInt16,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "mG",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    // Calibrated accel reading
    let field = FieldInfo {
        name: "compressed_calibrated_accel_y",
        field_type: FieldDataType::SInt16,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "mG",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    // Calibrated accel reading
    let field = FieldInfo {
        name: "compressed_calibrated_accel_z",
        field_type: FieldDataType::SInt16,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "mG",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    // Whole second part of the timestamp
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "accelerometer_data",
        global_message_number: MesgNum::AccelerometerData,
        fields,
    }
}
pub fn magnetometer_data_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Millisecond part of the timestamp.
    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // Each time in the array describes the time at which the compass sample with the corrosponding index was taken. Limited to 30 samples in each message. The samples may span across seconds. Array size must match the number of samples in cmps_x and cmps_y and cmps_z
    let field = FieldInfo {
        name: "sample_time_offset",
        field_type: FieldDataType::UInt16,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // These are the raw ADC reading. Maximum number of samples is 30 in each message. The samples may span across seconds. A conversion will need to be done on this data once read.
    let field = FieldInfo {
        name: "mag_x",
        field_type: FieldDataType::UInt16,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // These are the raw ADC reading. Maximum number of samples is 30 in each message. The samples may span across seconds. A conversion will need to be done on this data once read.
    let field = FieldInfo {
        name: "mag_y",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // These are the raw ADC reading. Maximum number of samples is 30 in each message. The samples may span across seconds. A conversion will need to be done on this data once read.
    let field = FieldInfo {
        name: "mag_z",
        field_type: FieldDataType::UInt16,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // Calibrated Magnetometer reading
    let field = FieldInfo {
        name: "calibrated_mag_x",
        field_type: FieldDataType::Float32,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "G",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // Calibrated Magnetometer reading
    let field = FieldInfo {
        name: "calibrated_mag_y",
        field_type: FieldDataType::Float32,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "G",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    // Calibrated Magnetometer reading
    let field = FieldInfo {
        name: "calibrated_mag_z",
        field_type: FieldDataType::Float32,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "G",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    // Whole second part of the timestamp
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "magnetometer_data",
        global_message_number: MesgNum::MagnetometerData,
        fields,
    }
}
pub fn barometer_data_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Millisecond part of the timestamp.
    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // Each time in the array describes the time at which the barometer sample with the corrosponding index was taken. The samples may span across seconds. Array size must match the number of samples in baro_cal
    let field = FieldInfo {
        name: "sample_time_offset",
        field_type: FieldDataType::UInt16,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // These are the raw ADC reading. The samples may span across seconds. A conversion will need to be done on this data once read.
    let field = FieldInfo {
        name: "baro_pres",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "Pa",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // Whole second part of the timestamp
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "barometer_data",
        global_message_number: MesgNum::BarometerData,
        fields,
    }
}
pub fn three_d_sensor_calibration_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Indicates which sensor the calibration is for
    let field = FieldInfo {
        name: "sensor_type",
        field_type: FieldDataType::SensorType,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let mut subfields = Vec::new();
    // Accelerometer calibration factor
    let sub_fld = FieldInfo {
        name: "accel_cal_factor",
        field_type: FieldDataType::UInt32,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "g",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, SensorType::Accelerometer.as_i64(), sub_fld));
    // Gyro calibration factor
    let sub_fld = FieldInfo {
        name: "gyro_cal_factor",
        field_type: FieldDataType::UInt32,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "deg/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, SensorType::Gyroscope.as_i64(), sub_fld));
    // Calibration factor used to convert from raw ADC value to degrees, g,  etc.
    let field = FieldInfo {
        name: "calibration_factor",
        field_type: FieldDataType::UInt32,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Calibration factor divisor
    let field = FieldInfo {
        name: "calibration_divisor",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // Level shift value used to shift the ADC value back into range
    let field = FieldInfo {
        name: "level_shift",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Internal calibration factors, one for each: xy, yx, zx
    let field = FieldInfo {
        name: "offset_cal",
        field_type: FieldDataType::SInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // 3 x 3 rotation matrix (row major)
    let field = FieldInfo {
        name: "orientation_matrix",
        field_type: FieldDataType::SInt32,
        def_number: 5,
        scale: 65535.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // Whole second part of the timestamp
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "three_d_sensor_calibration",
        global_message_number: MesgNum::ThreeDSensorCalibration,
        fields,
    }
}
pub fn one_d_sensor_calibration_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Indicates which sensor the calibration is for
    let field = FieldInfo {
        name: "sensor_type",
        field_type: FieldDataType::SensorType,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let mut subfields = Vec::new();
    // Barometer calibration factor
    let sub_fld = FieldInfo {
        name: "baro_cal_factor",
        field_type: FieldDataType::UInt32,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "Pa",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, SensorType::Barometer.as_i64(), sub_fld));
    // Calibration factor used to convert from raw ADC value to degrees, g,  etc.
    let field = FieldInfo {
        name: "calibration_factor",
        field_type: FieldDataType::UInt32,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Calibration factor divisor
    let field = FieldInfo {
        name: "calibration_divisor",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "counts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // Level shift value used to shift the ADC value back into range
    let field = FieldInfo {
        name: "level_shift",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Internal Calibration factor
    let field = FieldInfo {
        name: "offset_cal",
        field_type: FieldDataType::SInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // Whole second part of the timestamp
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "one_d_sensor_calibration",
        global_message_number: MesgNum::OneDSensorCalibration,
        fields,
    }
}
pub fn video_frame_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Millisecond part of the timestamp.
    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // Number of the frame that the timestamp and timestamp_ms correlate to
    let field = FieldInfo {
        name: "frame_number",
        field_type: FieldDataType::UInt32,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Whole second part of the timestamp
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "video_frame",
        global_message_number: MesgNum::VideoFrame,
        fields,
    }
}
pub fn obdii_data_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Fractional part of timestamp, added to timestamp
    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // Offset of PID reading [i] from start_timestamp+start_timestamp_ms. Readings may span accross seconds.
    let field = FieldInfo {
        name: "time_offset",
        field_type: FieldDataType::UInt16,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Parameter ID
    let field = FieldInfo {
        name: "pid",
        field_type: FieldDataType::Byte,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // Raw parameter data
    let field = FieldInfo {
        name: "raw_data",
        field_type: FieldDataType::Byte,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Optional, data size of PID[i].  If not specified refer to SAE J1979.
    let field = FieldInfo {
        name: "pid_data_size",
        field_type: FieldDataType::UInt8,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // System time associated with sample expressed in ms, can be used instead of time_offset.  There will be a system_time value for each raw_data element.  For multibyte pids the system_time is repeated.
    let field = FieldInfo {
        name: "system_time",
        field_type: FieldDataType::UInt32,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // Timestamp of first sample recorded in the message.  Used with time_offset to generate time of each sample
    let field = FieldInfo {
        name: "start_timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    // Fractional part of start_timestamp
    let field = FieldInfo {
        name: "start_timestamp_ms",
        field_type: FieldDataType::UInt16,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    // Timestamp message was output
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "obdii_data",
        global_message_number: MesgNum::ObdiiData,
        fields,
    }
}
pub fn nmea_sentence_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Fractional part of timestamp, added to timestamp
    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // NMEA sentence
    let field = FieldInfo {
        name: "sentence",
        field_type: FieldDataType::String,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Timestamp message was output
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "nmea_sentence",
        global_message_number: MesgNum::NmeaSentence,
        fields,
    }
}
pub fn aviation_attitude_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Fractional part of timestamp, added to timestamp
    let field = FieldInfo {
        name: "timestamp_ms",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // System time associated with sample expressed in ms.
    let field = FieldInfo {
        name: "system_time",
        field_type: FieldDataType::UInt32,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Range -PI/2 to +PI/2
    let field = FieldInfo {
        name: "pitch",
        field_type: FieldDataType::SInt16,
        def_number: 2,
        scale: 10430.380000,
        offset: 0.000000,
        units: "radians",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // Range -PI to +PI
    let field = FieldInfo {
        name: "roll",
        field_type: FieldDataType::SInt16,
        def_number: 3,
        scale: 10430.380000,
        offset: 0.000000,
        units: "radians",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Range -78.4 to +78.4 (-8 Gs to 8 Gs)
    let field = FieldInfo {
        name: "accel_lateral",
        field_type: FieldDataType::SInt16,
        def_number: 4,
        scale: 100.000000,
        offset: 0.000000,
        units: "m/s^2",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // Range -78.4 to +78.4 (-8 Gs to 8 Gs)
    let field = FieldInfo {
        name: "accel_normal",
        field_type: FieldDataType::SInt16,
        def_number: 5,
        scale: 100.000000,
        offset: 0.000000,
        units: "m/s^2",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // Range -8.727 to +8.727 (-500 degs/sec to +500 degs/sec)
    let field = FieldInfo {
        name: "turn_rate",
        field_type: FieldDataType::SInt16,
        def_number: 6,
        scale: 1024.000000,
        offset: 0.000000,
        units: "radians/second",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "stage",
        field_type: FieldDataType::AttitudeStage,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    // The percent complete of the current attitude stage.  Set to 0 for attitude stages 0, 1 and 2 and to 100 for attitude stage 3 by AHRS modules that do not support it.  Range - 100
    let field = FieldInfo {
        name: "attitude_stage_complete",
        field_type: FieldDataType::UInt8,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    // Track Angle/Heading Range 0 - 2pi
    let field = FieldInfo {
        name: "track",
        field_type: FieldDataType::UInt16,
        def_number: 9,
        scale: 10430.380000,
        offset: 0.000000,
        units: "radians",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "validity",
        field_type: FieldDataType::AttitudeValidity,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    // Timestamp message was output
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "aviation_attitude",
        global_message_number: MesgNum::AviationAttitude,
        fields,
    }
}
pub fn video_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "url",
        field_type: FieldDataType::String,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "hosting_provider",
        field_type: FieldDataType::String,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Playback time of video
    let field = FieldInfo {
        name: "duration",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    MessageInfo {
        name: "video",
        global_message_number: MesgNum::Video,
        fields,
    }
}
pub fn video_title_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Total number of title parts
    let field = FieldInfo {
        name: "message_count",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "text",
        field_type: FieldDataType::String,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Long titles will be split into multiple parts
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "video_title",
        global_message_number: MesgNum::VideoTitle,
        fields,
    }
}
pub fn video_description_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Total number of description parts
    let field = FieldInfo {
        name: "message_count",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "text",
        field_type: FieldDataType::String,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Long descriptions will be split into multiple parts
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "video_description",
        global_message_number: MesgNum::VideoDescription,
        fields,
    }
}
pub fn video_clip_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "clip_number",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "start_timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "start_timestamp_ms",
        field_type: FieldDataType::UInt16,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "end_timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "end_timestamp_ms",
        field_type: FieldDataType::UInt16,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // Start of clip in video time
    let field = FieldInfo {
        name: "clip_start",
        field_type: FieldDataType::UInt32,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    // End of clip in video time
    let field = FieldInfo {
        name: "clip_end",
        field_type: FieldDataType::UInt32,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "ms",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    MessageInfo {
        name: "video_clip",
        global_message_number: MesgNum::VideoClip,
        fields,
    }
}
pub fn set_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "duration",
        field_type: FieldDataType::UInt32,
        def_number: 0,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // # of repitions of the movement
    let field = FieldInfo {
        name: "repetitions",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Amount of weight applied for the set
    let field = FieldInfo {
        name: "weight",
        field_type: FieldDataType::UInt16,
        def_number: 4,
        scale: 16.000000,
        offset: 0.000000,
        units: "kg",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "set_type",
        field_type: FieldDataType::SetType,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // Start time of the set
    let field = FieldInfo {
        name: "start_time",
        field_type: FieldDataType::DateTime,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "category",
        field_type: FieldDataType::ExerciseCategory,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    // Based on the associated category, see [category]_exercise_names
    let field = FieldInfo {
        name: "category_subtype",
        field_type: FieldDataType::UInt16,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "weight_display_unit",
        field_type: FieldDataType::FitBaseUnit,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "wkt_step_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    // Timestamp of the set
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "set",
        global_message_number: MesgNum::Set,
        fields,
    }
}
pub fn jump_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "distance",
        field_type: FieldDataType::Float32,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "height",
        field_type: FieldDataType::Float32,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "rotations",
        field_type: FieldDataType::UInt8,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "hang_time",
        field_type: FieldDataType::Float32,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // A score for a jump calculated based on hang time, rotations, and distance.
    let field = FieldInfo {
        name: "score",
        field_type: FieldDataType::Float32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "position_lat",
        field_type: FieldDataType::SInt32,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "position_long",
        field_type: FieldDataType::SInt32,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 8,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        bits: 16,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "speed",
        field_type: FieldDataType::UInt16,
        def_number: 7,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(7, field);
    let field = FieldInfo {
        name: "enhanced_speed",
        field_type: FieldDataType::UInt32,
        def_number: 8,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "jump",
        global_message_number: MesgNum::Jump,
        fields,
    }
}
pub fn course_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "sport",
        field_type: FieldDataType::Sport,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "name",
        field_type: FieldDataType::String,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "capabilities",
        field_type: FieldDataType::CourseCapabilities,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "sub_sport",
        field_type: FieldDataType::SubSport,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    MessageInfo {
        name: "course",
        global_message_number: MesgNum::Course,
        fields,
    }
}
pub fn course_point_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "position_lat",
        field_type: FieldDataType::SInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "position_long",
        field_type: FieldDataType::SInt32,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "distance",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "type",
        field_type: FieldDataType::CoursePoint,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "name",
        field_type: FieldDataType::String,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "favorite",
        field_type: FieldDataType::Bool,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "course_point",
        global_message_number: MesgNum::CoursePoint,
        fields,
    }
}
/// Unique Identification data for a segment file
pub fn segment_id_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Friendly name assigned to segment
    let field = FieldInfo {
        name: "name",
        field_type: FieldDataType::String,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // UUID of the segment
    let field = FieldInfo {
        name: "uuid",
        field_type: FieldDataType::String,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Sport associated with the segment
    let field = FieldInfo {
        name: "sport",
        field_type: FieldDataType::Sport,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // Segment enabled for evaluation
    let field = FieldInfo {
        name: "enabled",
        field_type: FieldDataType::Bool,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Primary key of the user that created the segment
    let field = FieldInfo {
        name: "user_profile_primary_key",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // ID of the device that created the segment
    let field = FieldInfo {
        name: "device_id",
        field_type: FieldDataType::UInt32,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // Index for the Leader Board entry selected as the default race participant
    let field = FieldInfo {
        name: "default_race_leader",
        field_type: FieldDataType::UInt8,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    // Indicates if any segments should be deleted
    let field = FieldInfo {
        name: "delete_status",
        field_type: FieldDataType::SegmentDeleteStatus,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    // Indicates how the segment was selected to be sent to the device
    let field = FieldInfo {
        name: "selection_type",
        field_type: FieldDataType::SegmentSelectionType,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    MessageInfo {
        name: "segment_id",
        global_message_number: MesgNum::SegmentId,
        fields,
    }
}
/// Unique Identification data for an individual segment leader within a segment file
pub fn segment_leaderboard_entry_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Friendly name assigned to leader
    let field = FieldInfo {
        name: "name",
        field_type: FieldDataType::String,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // Leader classification
    let field = FieldInfo {
        name: "type",
        field_type: FieldDataType::SegmentLeaderboardType,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Primary user ID of this leader
    let field = FieldInfo {
        name: "group_primary_key",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // ID of the activity associated with this leader time
    let field = FieldInfo {
        name: "activity_id",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Segment Time (includes pauses)
    let field = FieldInfo {
        name: "segment_time",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // String version of the activity_id. 21 characters long, express in decimal
    let field = FieldInfo {
        name: "activity_id_string",
        field_type: FieldDataType::String,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "segment_leaderboard_entry",
        global_message_number: MesgNum::SegmentLeaderboardEntry,
        fields,
    }
}
/// Navigation and race evaluation point for a segment decribing a point along the segment path and time it took each segment leader to reach that point
pub fn segment_point_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "position_lat",
        field_type: FieldDataType::SInt32,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "position_long",
        field_type: FieldDataType::SInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // Accumulated distance along the segment at the described point
    let field = FieldInfo {
        name: "distance",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Accumulated altitude along the segment at the described point
    let field = FieldInfo {
        name: "altitude",
        field_type: FieldDataType::UInt16,
        def_number: 4,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // Accumualted time each leader board member required to reach the described point. This value is zero for all leader board members at the starting point of the segment.
    let field = FieldInfo {
        name: "leader_time",
        field_type: FieldDataType::UInt32,
        def_number: 5,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "segment_point",
        global_message_number: MesgNum::SegmentPoint,
        fields,
    }
}
pub fn segment_lap_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "event",
        field_type: FieldDataType::Event,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "event_type",
        field_type: FieldDataType::EventType,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "start_time",
        field_type: FieldDataType::DateTime,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "start_position_lat",
        field_type: FieldDataType::SInt32,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "start_position_long",
        field_type: FieldDataType::SInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "end_position_lat",
        field_type: FieldDataType::SInt32,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "end_position_long",
        field_type: FieldDataType::SInt32,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    // Time (includes pauses)
    let field = FieldInfo {
        name: "total_elapsed_time",
        field_type: FieldDataType::UInt32,
        def_number: 7,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    // Timer Time (excludes pauses)
    let field = FieldInfo {
        name: "total_timer_time",
        field_type: FieldDataType::UInt32,
        def_number: 8,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "total_distance",
        field_type: FieldDataType::UInt32,
        def_number: 9,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "total_strokes",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "strokes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((23, Sport::Cycling.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "total_cycles",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "cycles",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "total_calories",
        field_type: FieldDataType::UInt16,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    // If New Leaf
    let field = FieldInfo {
        name: "total_fat_calories",
        field_type: FieldDataType::UInt16,
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(12, field);
    let field = FieldInfo {
        name: "avg_speed",
        field_type: FieldDataType::UInt16,
        def_number: 13,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(13, field);
    let field = FieldInfo {
        name: "max_speed",
        field_type: FieldDataType::UInt16,
        def_number: 14,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(14, field);
    let field = FieldInfo {
        name: "avg_heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 15,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(15, field);
    let field = FieldInfo {
        name: "max_heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 16,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(16, field);
    // total_cycles / total_timer_time if non_zero_avg_cadence otherwise total_cycles / total_elapsed_time
    let field = FieldInfo {
        name: "avg_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 17,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(17, field);
    let field = FieldInfo {
        name: "max_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 18,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(18, field);
    // total_power / total_timer_time if non_zero_avg_power otherwise total_power / total_elapsed_time
    let field = FieldInfo {
        name: "avg_power",
        field_type: FieldDataType::UInt16,
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(19, field);
    let field = FieldInfo {
        name: "max_power",
        field_type: FieldDataType::UInt16,
        def_number: 20,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(20, field);
    let field = FieldInfo {
        name: "total_ascent",
        field_type: FieldDataType::UInt16,
        def_number: 21,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(21, field);
    let field = FieldInfo {
        name: "total_descent",
        field_type: FieldDataType::UInt16,
        def_number: 22,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(22, field);
    let field = FieldInfo {
        name: "sport",
        field_type: FieldDataType::Sport,
        def_number: 23,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(23, field);
    let field = FieldInfo {
        name: "event_group",
        field_type: FieldDataType::UInt8,
        def_number: 24,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(24, field);
    // North east corner latitude.
    let field = FieldInfo {
        name: "nec_lat",
        field_type: FieldDataType::SInt32,
        def_number: 25,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(25, field);
    // North east corner longitude.
    let field = FieldInfo {
        name: "nec_long",
        field_type: FieldDataType::SInt32,
        def_number: 26,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(26, field);
    // South west corner latitude.
    let field = FieldInfo {
        name: "swc_lat",
        field_type: FieldDataType::SInt32,
        def_number: 27,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(27, field);
    // South west corner latitude.
    let field = FieldInfo {
        name: "swc_long",
        field_type: FieldDataType::SInt32,
        def_number: 28,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(28, field);
    let field = FieldInfo {
        name: "name",
        field_type: FieldDataType::String,
        def_number: 29,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(29, field);
    let field = FieldInfo {
        name: "normalized_power",
        field_type: FieldDataType::UInt16,
        def_number: 30,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(30, field);
    let field = FieldInfo {
        name: "left_right_balance",
        field_type: FieldDataType::LeftRightBalance100,
        def_number: 31,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(31, field);
    let field = FieldInfo {
        name: "sub_sport",
        field_type: FieldDataType::SubSport,
        def_number: 32,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(32, field);
    let field = FieldInfo {
        name: "total_work",
        field_type: FieldDataType::UInt32,
        def_number: 33,
        scale: 1.000000,
        offset: 0.000000,
        units: "J",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(33, field);
    let field = FieldInfo {
        name: "avg_altitude",
        field_type: FieldDataType::UInt16,
        def_number: 34,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(34, field);
    let field = FieldInfo {
        name: "max_altitude",
        field_type: FieldDataType::UInt16,
        def_number: 35,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(35, field);
    let field = FieldInfo {
        name: "gps_accuracy",
        field_type: FieldDataType::UInt8,
        def_number: 36,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(36, field);
    let field = FieldInfo {
        name: "avg_grade",
        field_type: FieldDataType::SInt16,
        def_number: 37,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(37, field);
    let field = FieldInfo {
        name: "avg_pos_grade",
        field_type: FieldDataType::SInt16,
        def_number: 38,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(38, field);
    let field = FieldInfo {
        name: "avg_neg_grade",
        field_type: FieldDataType::SInt16,
        def_number: 39,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(39, field);
    let field = FieldInfo {
        name: "max_pos_grade",
        field_type: FieldDataType::SInt16,
        def_number: 40,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(40, field);
    let field = FieldInfo {
        name: "max_neg_grade",
        field_type: FieldDataType::SInt16,
        def_number: 41,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(41, field);
    let field = FieldInfo {
        name: "avg_temperature",
        field_type: FieldDataType::SInt8,
        def_number: 42,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(42, field);
    let field = FieldInfo {
        name: "max_temperature",
        field_type: FieldDataType::SInt8,
        def_number: 43,
        scale: 1.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(43, field);
    let field = FieldInfo {
        name: "total_moving_time",
        field_type: FieldDataType::UInt32,
        def_number: 44,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(44, field);
    let field = FieldInfo {
        name: "avg_pos_vertical_speed",
        field_type: FieldDataType::SInt16,
        def_number: 45,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(45, field);
    let field = FieldInfo {
        name: "avg_neg_vertical_speed",
        field_type: FieldDataType::SInt16,
        def_number: 46,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(46, field);
    let field = FieldInfo {
        name: "max_pos_vertical_speed",
        field_type: FieldDataType::SInt16,
        def_number: 47,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(47, field);
    let field = FieldInfo {
        name: "max_neg_vertical_speed",
        field_type: FieldDataType::SInt16,
        def_number: 48,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(48, field);
    let field = FieldInfo {
        name: "time_in_hr_zone",
        field_type: FieldDataType::UInt32,
        def_number: 49,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(49, field);
    let field = FieldInfo {
        name: "time_in_speed_zone",
        field_type: FieldDataType::UInt32,
        def_number: 50,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(50, field);
    let field = FieldInfo {
        name: "time_in_cadence_zone",
        field_type: FieldDataType::UInt32,
        def_number: 51,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(51, field);
    let field = FieldInfo {
        name: "time_in_power_zone",
        field_type: FieldDataType::UInt32,
        def_number: 52,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(52, field);
    let field = FieldInfo {
        name: "repetition_num",
        field_type: FieldDataType::UInt16,
        def_number: 53,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(53, field);
    let field = FieldInfo {
        name: "min_altitude",
        field_type: FieldDataType::UInt16,
        def_number: 54,
        scale: 5.000000,
        offset: 500.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(54, field);
    let field = FieldInfo {
        name: "min_heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 55,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(55, field);
    let field = FieldInfo {
        name: "active_time",
        field_type: FieldDataType::UInt32,
        def_number: 56,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(56, field);
    let field = FieldInfo {
        name: "wkt_step_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 57,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(57, field);
    let field = FieldInfo {
        name: "sport_event",
        field_type: FieldDataType::SportEvent,
        def_number: 58,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(58, field);
    let field = FieldInfo {
        name: "avg_left_torque_effectiveness",
        field_type: FieldDataType::UInt8,
        def_number: 59,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(59, field);
    let field = FieldInfo {
        name: "avg_right_torque_effectiveness",
        field_type: FieldDataType::UInt8,
        def_number: 60,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(60, field);
    let field = FieldInfo {
        name: "avg_left_pedal_smoothness",
        field_type: FieldDataType::UInt8,
        def_number: 61,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(61, field);
    let field = FieldInfo {
        name: "avg_right_pedal_smoothness",
        field_type: FieldDataType::UInt8,
        def_number: 62,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(62, field);
    let field = FieldInfo {
        name: "avg_combined_pedal_smoothness",
        field_type: FieldDataType::UInt8,
        def_number: 63,
        scale: 2.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(63, field);
    let field = FieldInfo {
        name: "status",
        field_type: FieldDataType::SegmentLapStatus,
        def_number: 64,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(64, field);
    let field = FieldInfo {
        name: "uuid",
        field_type: FieldDataType::String,
        def_number: 65,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(65, field);
    // fractional part of the avg_cadence
    let field = FieldInfo {
        name: "avg_fractional_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 66,
        scale: 128.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(66, field);
    // fractional part of the max_cadence
    let field = FieldInfo {
        name: "max_fractional_cadence",
        field_type: FieldDataType::UInt8,
        def_number: 67,
        scale: 128.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(67, field);
    // fractional part of the total_cycles
    let field = FieldInfo {
        name: "total_fractional_cycles",
        field_type: FieldDataType::UInt8,
        def_number: 68,
        scale: 128.000000,
        offset: 0.000000,
        units: "cycles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(68, field);
    let field = FieldInfo {
        name: "front_gear_shift_count",
        field_type: FieldDataType::UInt16,
        def_number: 69,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(69, field);
    let field = FieldInfo {
        name: "rear_gear_shift_count",
        field_type: FieldDataType::UInt16,
        def_number: 70,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(70, field);
    // Total time spent in the standing position
    let field = FieldInfo {
        name: "time_standing",
        field_type: FieldDataType::UInt32,
        def_number: 71,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(71, field);
    // Number of transitions to the standing state
    let field = FieldInfo {
        name: "stand_count",
        field_type: FieldDataType::UInt16,
        def_number: 72,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(72, field);
    // Average left platform center offset
    let field = FieldInfo {
        name: "avg_left_pco",
        field_type: FieldDataType::SInt8,
        def_number: 73,
        scale: 1.000000,
        offset: 0.000000,
        units: "mm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(73, field);
    // Average right platform center offset
    let field = FieldInfo {
        name: "avg_right_pco",
        field_type: FieldDataType::SInt8,
        def_number: 74,
        scale: 1.000000,
        offset: 0.000000,
        units: "mm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(74, field);
    // Average left power phase angles. Data value indexes defined by power_phase_type.
    let field = FieldInfo {
        name: "avg_left_power_phase",
        field_type: FieldDataType::UInt8,
        def_number: 75,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(75, field);
    // Average left power phase peak angles. Data value indexes defined by power_phase_type.
    let field = FieldInfo {
        name: "avg_left_power_phase_peak",
        field_type: FieldDataType::UInt8,
        def_number: 76,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(76, field);
    // Average right power phase angles. Data value indexes defined by power_phase_type.
    let field = FieldInfo {
        name: "avg_right_power_phase",
        field_type: FieldDataType::UInt8,
        def_number: 77,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(77, field);
    // Average right power phase peak angles. Data value indexes defined by power_phase_type.
    let field = FieldInfo {
        name: "avg_right_power_phase_peak",
        field_type: FieldDataType::UInt8,
        def_number: 78,
        scale: 0.711111,
        offset: 0.000000,
        units: "degrees",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(78, field);
    // Average power by position. Data value indexes defined by rider_position_type.
    let field = FieldInfo {
        name: "avg_power_position",
        field_type: FieldDataType::UInt16,
        def_number: 79,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(79, field);
    // Maximum power by position. Data value indexes defined by rider_position_type.
    let field = FieldInfo {
        name: "max_power_position",
        field_type: FieldDataType::UInt16,
        def_number: 80,
        scale: 1.000000,
        offset: 0.000000,
        units: "watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(80, field);
    // Average cadence by position. Data value indexes defined by rider_position_type.
    let field = FieldInfo {
        name: "avg_cadence_position",
        field_type: FieldDataType::UInt8,
        def_number: 81,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(81, field);
    // Maximum cadence by position. Data value indexes defined by rider_position_type.
    let field = FieldInfo {
        name: "max_cadence_position",
        field_type: FieldDataType::UInt8,
        def_number: 82,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(82, field);
    // Manufacturer that produced the segment
    let field = FieldInfo {
        name: "manufacturer",
        field_type: FieldDataType::Manufacturer,
        def_number: 83,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(83, field);
    // The grit score estimates how challenging a route could be for a cyclist in terms of time spent going over sharp turns or large grade slopes.
    let field = FieldInfo {
        name: "total_grit",
        field_type: FieldDataType::Float32,
        def_number: 84,
        scale: 1.000000,
        offset: 0.000000,
        units: "kGrit",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(84, field);
    // The flow score estimates how long distance wise a cyclist deaccelerates over intervals where deacceleration is unnecessary such as smooth turns or small grade angle intervals.
    let field = FieldInfo {
        name: "total_flow",
        field_type: FieldDataType::Float32,
        def_number: 85,
        scale: 1.000000,
        offset: 0.000000,
        units: "Flow",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(85, field);
    // The grit score estimates how challenging a route could be for a cyclist in terms of time spent going over sharp turns or large grade slopes.
    let field = FieldInfo {
        name: "avg_grit",
        field_type: FieldDataType::Float32,
        def_number: 86,
        scale: 1.000000,
        offset: 0.000000,
        units: "kGrit",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(86, field);
    // The flow score estimates how long distance wise a cyclist deaccelerates over intervals where deacceleration is unnecessary such as smooth turns or small grade angle intervals.
    let field = FieldInfo {
        name: "avg_flow",
        field_type: FieldDataType::Float32,
        def_number: 87,
        scale: 1.000000,
        offset: 0.000000,
        units: "Flow",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(87, field);
    // fractional part of total_ascent
    let field = FieldInfo {
        name: "total_fractional_ascent",
        field_type: FieldDataType::UInt8,
        def_number: 89,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(89, field);
    // fractional part of total_descent
    let field = FieldInfo {
        name: "total_fractional_descent",
        field_type: FieldDataType::UInt8,
        def_number: 90,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(90, field);
    // Lap end time.
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "segment_lap",
        global_message_number: MesgNum::SegmentLap,
        fields,
    }
}
/// Summary of the unique segment and leaderboard information associated with a segment file. This message is used to compile a segment list file describing all segment files on a device. The segment list file is used when refreshing the contents of a segment file with the latest available leaderboard information.
pub fn segment_file_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // UUID of the segment file
    let field = FieldInfo {
        name: "file_uuid",
        field_type: FieldDataType::String,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Enabled state of the segment file
    let field = FieldInfo {
        name: "enabled",
        field_type: FieldDataType::Bool,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Primary key of the user that created the segment file
    let field = FieldInfo {
        name: "user_profile_primary_key",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    // Leader type of each leader in the segment file
    let field = FieldInfo {
        name: "leader_type",
        field_type: FieldDataType::SegmentLeaderboardType,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    // Group primary key of each leader in the segment file
    let field = FieldInfo {
        name: "leader_group_primary_key",
        field_type: FieldDataType::UInt32,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    // Activity ID of each leader in the segment file
    let field = FieldInfo {
        name: "leader_activity_id",
        field_type: FieldDataType::UInt32,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    // String version of the activity ID of each leader in the segment file. 21 characters long for each ID, express in decimal
    let field = FieldInfo {
        name: "leader_activity_id_string",
        field_type: FieldDataType::String,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    // Index for the Leader Board entry selected as the default race participant
    let field = FieldInfo {
        name: "default_race_leader",
        field_type: FieldDataType::UInt8,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "segment_file",
        global_message_number: MesgNum::SegmentFile,
        fields,
    }
}
pub fn workout_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "sport",
        field_type: FieldDataType::Sport,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "capabilities",
        field_type: FieldDataType::WorkoutCapabilities,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    // number of valid steps
    let field = FieldInfo {
        name: "num_valid_steps",
        field_type: FieldDataType::UInt16,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "wkt_name",
        field_type: FieldDataType::String,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "sub_sport",
        field_type: FieldDataType::SubSport,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    let field = FieldInfo {
        name: "pool_length",
        field_type: FieldDataType::UInt16,
        def_number: 14,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(14, field);
    let field = FieldInfo {
        name: "pool_length_unit",
        field_type: FieldDataType::DisplayMeasure,
        def_number: 15,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(15, field);
    MessageInfo {
        name: "workout",
        global_message_number: MesgNum::Workout,
        fields,
    }
}
pub fn workout_session_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "sport",
        field_type: FieldDataType::Sport,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "sub_sport",
        field_type: FieldDataType::SubSport,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "num_valid_steps",
        field_type: FieldDataType::UInt16,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "first_step_index",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "pool_length",
        field_type: FieldDataType::UInt16,
        def_number: 4,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "pool_length_unit",
        field_type: FieldDataType::DisplayMeasure,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "workout_session",
        global_message_number: MesgNum::WorkoutSession,
        fields,
    }
}
pub fn workout_step_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "wkt_step_name",
        field_type: FieldDataType::String,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "duration_type",
        field_type: FieldDataType::WktStepDuration,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "duration_time",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::Time.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "duration_time",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::RepetitionTime.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "duration_distance",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::Distance.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "duration_hr",
        field_type: FieldDataType::WorkoutHr,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "% or bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::HrLessThan.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "duration_hr",
        field_type: FieldDataType::WorkoutHr,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "% or bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::HrGreaterThan.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "duration_calories",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "calories",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::Calories.as_i64(), sub_fld));
    // message_index of step to loop back to. Steps are assumed to be in the order by message_index. custom_name and intensity members are undefined for this duration type.
    let sub_fld = FieldInfo {
        name: "duration_step",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::RepeatUntilStepsCmplt.as_i64(), sub_fld));
    // message_index of step to loop back to. Steps are assumed to be in the order by message_index. custom_name and intensity members are undefined for this duration type.
    let sub_fld = FieldInfo {
        name: "duration_step",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::RepeatUntilTime.as_i64(), sub_fld));
    // message_index of step to loop back to. Steps are assumed to be in the order by message_index. custom_name and intensity members are undefined for this duration type.
    let sub_fld = FieldInfo {
        name: "duration_step",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::RepeatUntilDistance.as_i64(), sub_fld));
    // message_index of step to loop back to. Steps are assumed to be in the order by message_index. custom_name and intensity members are undefined for this duration type.
    let sub_fld = FieldInfo {
        name: "duration_step",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::RepeatUntilCalories.as_i64(), sub_fld));
    // message_index of step to loop back to. Steps are assumed to be in the order by message_index. custom_name and intensity members are undefined for this duration type.
    let sub_fld = FieldInfo {
        name: "duration_step",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::RepeatUntilHrLessThan.as_i64(), sub_fld));
    // message_index of step to loop back to. Steps are assumed to be in the order by message_index. custom_name and intensity members are undefined for this duration type.
    let sub_fld = FieldInfo {
        name: "duration_step",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((
        1,
        WktStepDuration::RepeatUntilHrGreaterThan.as_i64(),
        sub_fld,
    ));
    // message_index of step to loop back to. Steps are assumed to be in the order by message_index. custom_name and intensity members are undefined for this duration type.
    let sub_fld = FieldInfo {
        name: "duration_step",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((
        1,
        WktStepDuration::RepeatUntilPowerLessThan.as_i64(),
        sub_fld,
    ));
    // message_index of step to loop back to. Steps are assumed to be in the order by message_index. custom_name and intensity members are undefined for this duration type.
    let sub_fld = FieldInfo {
        name: "duration_step",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((
        1,
        WktStepDuration::RepeatUntilPowerGreaterThan.as_i64(),
        sub_fld,
    ));
    let sub_fld = FieldInfo {
        name: "duration_power",
        field_type: FieldDataType::WorkoutPower,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "% or watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::PowerLessThan.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "duration_power",
        field_type: FieldDataType::WorkoutPower,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "% or watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::PowerGreaterThan.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "duration_reps",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::Reps.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "duration_value",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "target_type",
        field_type: FieldDataType::WktStepTarget,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let mut subfields = Vec::new();
    // speed zone (1-10);Custom =0;
    let sub_fld = FieldInfo {
        name: "target_speed_zone",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((3, WktStepTarget::Speed.as_i64(), sub_fld));
    // hr zone (1-5);Custom =0;
    let sub_fld = FieldInfo {
        name: "target_hr_zone",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((3, WktStepTarget::HeartRate.as_i64(), sub_fld));
    // Zone (1-?); Custom = 0;
    let sub_fld = FieldInfo {
        name: "target_cadence_zone",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((3, WktStepTarget::Cadence.as_i64(), sub_fld));
    // Power Zone ( 1-7); Custom = 0;
    let sub_fld = FieldInfo {
        name: "target_power_zone",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((3, WktStepTarget::Power.as_i64(), sub_fld));
    // # of repetitions
    let sub_fld = FieldInfo {
        name: "repeat_steps",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::RepeatUntilStepsCmplt.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "repeat_time",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::RepeatUntilTime.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "repeat_distance",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::RepeatUntilDistance.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "repeat_calories",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "calories",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::RepeatUntilCalories.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "repeat_hr",
        field_type: FieldDataType::WorkoutHr,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "% or bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((1, WktStepDuration::RepeatUntilHrLessThan.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "repeat_hr",
        field_type: FieldDataType::WorkoutHr,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "% or bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((
        1,
        WktStepDuration::RepeatUntilHrGreaterThan.as_i64(),
        sub_fld,
    ));
    let sub_fld = FieldInfo {
        name: "repeat_power",
        field_type: FieldDataType::WorkoutPower,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "% or watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((
        1,
        WktStepDuration::RepeatUntilPowerLessThan.as_i64(),
        sub_fld,
    ));
    let sub_fld = FieldInfo {
        name: "repeat_power",
        field_type: FieldDataType::WorkoutPower,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "% or watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((
        1,
        WktStepDuration::RepeatUntilPowerGreaterThan.as_i64(),
        sub_fld,
    ));
    let sub_fld = FieldInfo {
        name: "target_stroke_type",
        field_type: FieldDataType::SwimStroke,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((3, WktStepTarget::SwimStroke.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "target_value",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(4, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "custom_target_speed_low",
        field_type: FieldDataType::UInt32,
        def_number: 5,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((3, WktStepTarget::Speed.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "custom_target_heart_rate_low",
        field_type: FieldDataType::WorkoutHr,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "% or bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((3, WktStepTarget::HeartRate.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "custom_target_cadence_low",
        field_type: FieldDataType::UInt32,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((3, WktStepTarget::Cadence.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "custom_target_power_low",
        field_type: FieldDataType::WorkoutPower,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "% or watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((3, WktStepTarget::Power.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "custom_target_value_low",
        field_type: FieldDataType::UInt32,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(5, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "custom_target_speed_high",
        field_type: FieldDataType::UInt32,
        def_number: 6,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m/s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((3, WktStepTarget::Speed.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "custom_target_heart_rate_high",
        field_type: FieldDataType::WorkoutHr,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "% or bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((3, WktStepTarget::HeartRate.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "custom_target_cadence_high",
        field_type: FieldDataType::UInt32,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "rpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((3, WktStepTarget::Cadence.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "custom_target_power_high",
        field_type: FieldDataType::WorkoutPower,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "% or watts",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((3, WktStepTarget::Power.as_i64(), sub_fld));
    let field = FieldInfo {
        name: "custom_target_value_high",
        field_type: FieldDataType::UInt32,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "intensity",
        field_type: FieldDataType::Intensity,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    let field = FieldInfo {
        name: "notes",
        field_type: FieldDataType::String,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "equipment",
        field_type: FieldDataType::WorkoutEquipment,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "exercise_category",
        field_type: FieldDataType::ExerciseCategory,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "exercise_name",
        field_type: FieldDataType::UInt16,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    let field = FieldInfo {
        name: "exercise_weight",
        field_type: FieldDataType::UInt16,
        def_number: 12,
        scale: 100.000000,
        offset: 0.000000,
        units: "kg",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(12, field);
    let field = FieldInfo {
        name: "weight_display_unit",
        field_type: FieldDataType::FitBaseUnit,
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(13, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "workout_step",
        global_message_number: MesgNum::WorkoutStep,
        fields,
    }
}
pub fn exercise_title_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "exercise_category",
        field_type: FieldDataType::ExerciseCategory,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "exercise_name",
        field_type: FieldDataType::UInt16,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "wkt_step_name",
        field_type: FieldDataType::String,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "exercise_title",
        global_message_number: MesgNum::ExerciseTitle,
        fields,
    }
}
pub fn schedule_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Corresponds to file_id of scheduled workout / course.
    let field = FieldInfo {
        name: "manufacturer",
        field_type: FieldDataType::Manufacturer,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "favero_product",
        field_type: FieldDataType::FaveroProduct,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Manufacturer::FaveroElectronics.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Manufacturer::Garmin.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Manufacturer::Dynastream.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Manufacturer::DynastreamOem.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "garmin_product",
        field_type: FieldDataType::GarminProduct,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((0, Manufacturer::Tacx.as_i64(), sub_fld));
    // Corresponds to file_id of scheduled workout / course.
    let field = FieldInfo {
        name: "product",
        field_type: FieldDataType::UInt16,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Corresponds to file_id of scheduled workout / course.
    let field = FieldInfo {
        name: "serial_number",
        field_type: FieldDataType::UInt32z,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // Corresponds to file_id of scheduled workout / course.
    let field = FieldInfo {
        name: "time_created",
        field_type: FieldDataType::DateTime,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // TRUE if this activity has been started
    let field = FieldInfo {
        name: "completed",
        field_type: FieldDataType::Bool,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "type",
        field_type: FieldDataType::Schedule,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "scheduled_time",
        field_type: FieldDataType::LocalDateTime,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    MessageInfo {
        name: "schedule",
        global_message_number: MesgNum::Schedule,
        fields,
    }
}
pub fn totals_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Excludes pauses
    let field = FieldInfo {
        name: "timer_time",
        field_type: FieldDataType::UInt32,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "distance",
        field_type: FieldDataType::UInt32,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "calories",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "sport",
        field_type: FieldDataType::Sport,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Includes pauses
    let field = FieldInfo {
        name: "elapsed_time",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "sessions",
        field_type: FieldDataType::UInt16,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "active_time",
        field_type: FieldDataType::UInt32,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "sport_index",
        field_type: FieldDataType::UInt8,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 254,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(254, field);
    MessageInfo {
        name: "totals",
        global_message_number: MesgNum::Totals,
        fields,
    }
}
pub fn weight_scale_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "weight",
        field_type: FieldDataType::Weight,
        def_number: 0,
        scale: 100.000000,
        offset: 0.000000,
        units: "kg",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "percent_fat",
        field_type: FieldDataType::UInt16,
        def_number: 1,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "percent_hydration",
        field_type: FieldDataType::UInt16,
        def_number: 2,
        scale: 100.000000,
        offset: 0.000000,
        units: "%",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "visceral_fat_mass",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 100.000000,
        offset: 0.000000,
        units: "kg",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "bone_mass",
        field_type: FieldDataType::UInt16,
        def_number: 4,
        scale: 100.000000,
        offset: 0.000000,
        units: "kg",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "muscle_mass",
        field_type: FieldDataType::UInt16,
        def_number: 5,
        scale: 100.000000,
        offset: 0.000000,
        units: "kg",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "basal_met",
        field_type: FieldDataType::UInt16,
        def_number: 7,
        scale: 4.000000,
        offset: 0.000000,
        units: "kcal/day",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    let field = FieldInfo {
        name: "physique_rating",
        field_type: FieldDataType::UInt8,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    // ~4kJ per kcal, 0.25 allows max 16384 kcal
    let field = FieldInfo {
        name: "active_met",
        field_type: FieldDataType::UInt16,
        def_number: 9,
        scale: 4.000000,
        offset: 0.000000,
        units: "kcal/day",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "metabolic_age",
        field_type: FieldDataType::UInt8,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "years",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "visceral_fat_rating",
        field_type: FieldDataType::UInt8,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    // Associates this weight scale message to a user.  This corresponds to the index of the user profile message in the weight scale file.
    let field = FieldInfo {
        name: "user_profile_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 12,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(12, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "weight_scale",
        global_message_number: MesgNum::WeightScale,
        fields,
    }
}
pub fn blood_pressure_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "systolic_pressure",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "mmHg",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "diastolic_pressure",
        field_type: FieldDataType::UInt16,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "mmHg",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "mean_arterial_pressure",
        field_type: FieldDataType::UInt16,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "mmHg",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "map_3_sample_mean",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "mmHg",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "map_morning_values",
        field_type: FieldDataType::UInt16,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "mmHg",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "map_evening_values",
        field_type: FieldDataType::UInt16,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "mmHg",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "heart_rate_type",
        field_type: FieldDataType::HrType,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    let field = FieldInfo {
        name: "status",
        field_type: FieldDataType::BpStatus,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    // Associates this blood pressure message to a user.  This corresponds to the index of the user profile message in the blood pressure file.
    let field = FieldInfo {
        name: "user_profile_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "blood_pressure",
        global_message_number: MesgNum::BloodPressure,
        fields,
    }
}
pub fn monitoring_info_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Use to convert activity timestamps to local time if device does not support time zone and daylight savings time correction.
    let field = FieldInfo {
        name: "local_timestamp",
        field_type: FieldDataType::LocalDateTime,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "activity_type",
        field_type: FieldDataType::ActivityType,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Indexed by activity_type
    let field = FieldInfo {
        name: "cycles_to_distance",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 5000.000000,
        offset: 0.000000,
        units: "m/cycle",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Indexed by activity_type
    let field = FieldInfo {
        name: "cycles_to_calories",
        field_type: FieldDataType::UInt16,
        def_number: 4,
        scale: 5000.000000,
        offset: 0.000000,
        units: "kcal/cycle",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "resting_metabolic_rate",
        field_type: FieldDataType::UInt16,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal / day",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "monitoring_info",
        global_message_number: MesgNum::MonitoringInfo,
        fields,
    }
}
pub fn monitoring_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Associates this data to device_info message.  Not required for file with single device (sensor).
    let field = FieldInfo {
        name: "device_index",
        field_type: FieldDataType::DeviceIndex,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // Accumulated total calories.  Maintained by MonitoringReader for each activity_type.  See SDK documentation
    let field = FieldInfo {
        name: "calories",
        field_type: FieldDataType::UInt16,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Accumulated distance.  Maintained by MonitoringReader for each activity_type.  See SDK documentation.
    let field = FieldInfo {
        name: "distance",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 100.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let mut subfields = Vec::new();
    let sub_fld = FieldInfo {
        name: "steps",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "steps",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((5, ActivityType::Walking.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "steps",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "steps",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((5, ActivityType::Running.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "strokes",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 2.000000,
        offset: 0.000000,
        units: "strokes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((5, ActivityType::Cycling.as_i64(), sub_fld));
    let sub_fld = FieldInfo {
        name: "strokes",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 2.000000,
        offset: 0.000000,
        units: "strokes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    subfields.push((5, ActivityType::Swimming.as_i64(), sub_fld));
    // Accumulated cycles.  Maintained by MonitoringReader for each activity_type.  See SDK documentation.
    let field = FieldInfo {
        name: "cycles",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 2.000000,
        offset: 0.000000,
        units: "cycles",
        accumulate: false,
        subfields: subfields,
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "active_time",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "activity_type",
        field_type: FieldDataType::ActivityType,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "activity_subtype",
        field_type: FieldDataType::ActivitySubtype,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "activity_level",
        field_type: FieldDataType::ActivityLevel,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    let field = FieldInfo {
        name: "distance_16",
        field_type: FieldDataType::UInt16,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "100 * m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "cycles_16",
        field_type: FieldDataType::UInt16,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "2 * cycles (steps)",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "active_time_16",
        field_type: FieldDataType::UInt16,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    // Must align to logging interval, for example, time must be 00:00:00 for daily log.
    let field = FieldInfo {
        name: "local_timestamp",
        field_type: FieldDataType::LocalDateTime,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    // Avg temperature during the logging interval ended at timestamp
    let field = FieldInfo {
        name: "temperature",
        field_type: FieldDataType::SInt16,
        def_number: 12,
        scale: 100.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(12, field);
    // Min temperature during the logging interval ended at timestamp
    let field = FieldInfo {
        name: "temperature_min",
        field_type: FieldDataType::SInt16,
        def_number: 14,
        scale: 100.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(14, field);
    // Max temperature during the logging interval ended at timestamp
    let field = FieldInfo {
        name: "temperature_max",
        field_type: FieldDataType::SInt16,
        def_number: 15,
        scale: 100.000000,
        offset: 0.000000,
        units: "C",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(15, field);
    // Indexed using minute_activity_level enum
    let field = FieldInfo {
        name: "activity_time",
        field_type: FieldDataType::UInt16,
        def_number: 16,
        scale: 1.000000,
        offset: 0.000000,
        units: "minutes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(16, field);
    let field = FieldInfo {
        name: "active_calories",
        field_type: FieldDataType::UInt16,
        def_number: 19,
        scale: 1.000000,
        offset: 0.000000,
        units: "kcal",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(19, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 5,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 28,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 3,
        accumulate: false,
    };
    components.push(comp_fld);
    // Indicates single type / intensity for duration since last monitoring message.
    let field = FieldInfo {
        name: "current_activity_type_intensity",
        field_type: FieldDataType::Byte,
        def_number: 24,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(24, field);
    let field = FieldInfo {
        name: "timestamp_min_8",
        field_type: FieldDataType::UInt8,
        def_number: 25,
        scale: 1.000000,
        offset: 0.000000,
        units: "min",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(25, field);
    let field = FieldInfo {
        name: "timestamp_16",
        field_type: FieldDataType::UInt16,
        def_number: 26,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(26, field);
    let field = FieldInfo {
        name: "heart_rate",
        field_type: FieldDataType::UInt8,
        def_number: 27,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(27, field);
    let field = FieldInfo {
        name: "intensity",
        field_type: FieldDataType::UInt8,
        def_number: 28,
        scale: 10.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(28, field);
    let field = FieldInfo {
        name: "duration_min",
        field_type: FieldDataType::UInt16,
        def_number: 29,
        scale: 1.000000,
        offset: 0.000000,
        units: "min",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(29, field);
    let field = FieldInfo {
        name: "duration",
        field_type: FieldDataType::UInt32,
        def_number: 30,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(30, field);
    let field = FieldInfo {
        name: "ascent",
        field_type: FieldDataType::UInt32,
        def_number: 31,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(31, field);
    let field = FieldInfo {
        name: "descent",
        field_type: FieldDataType::UInt32,
        def_number: 32,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(32, field);
    let field = FieldInfo {
        name: "moderate_activity_minutes",
        field_type: FieldDataType::UInt16,
        def_number: 33,
        scale: 1.000000,
        offset: 0.000000,
        units: "minutes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(33, field);
    let field = FieldInfo {
        name: "vigorous_activity_minutes",
        field_type: FieldDataType::UInt16,
        def_number: 34,
        scale: 1.000000,
        offset: 0.000000,
        units: "minutes",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(34, field);
    // Must align to logging interval, for example, time must be 00:00:00 for daily log.
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "monitoring",
        global_message_number: MesgNum::Monitoring,
        fields,
    }
}
pub fn hr_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "fractional_timestamp",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 32768.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 0,
        scale: 256.000000,
        offset: 0.000000,
        units: "s",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "time256",
        field_type: FieldDataType::UInt8,
        def_number: 1,
        scale: 256.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "filtered_bpm",
        field_type: FieldDataType::UInt8,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "bpm",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "event_timestamp",
        field_type: FieldDataType::UInt32,
        def_number: 9,
        scale: 1024.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 9,
        scale: 1024.000000,
        offset: 0.000000,
        units: "s",
        bits: 12,
        accumulate: true,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 9,
        scale: 1024.000000,
        offset: 0.000000,
        units: "",
        bits: 12,
        accumulate: true,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 9,
        scale: 1024.000000,
        offset: 0.000000,
        units: "",
        bits: 12,
        accumulate: true,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 9,
        scale: 1024.000000,
        offset: 0.000000,
        units: "",
        bits: 12,
        accumulate: true,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 9,
        scale: 1024.000000,
        offset: 0.000000,
        units: "",
        bits: 12,
        accumulate: true,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 9,
        scale: 1024.000000,
        offset: 0.000000,
        units: "",
        bits: 12,
        accumulate: true,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 9,
        scale: 1024.000000,
        offset: 0.000000,
        units: "",
        bits: 12,
        accumulate: true,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 9,
        scale: 1024.000000,
        offset: 0.000000,
        units: "",
        bits: 12,
        accumulate: true,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 9,
        scale: 1024.000000,
        offset: 0.000000,
        units: "",
        bits: 12,
        accumulate: true,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 9,
        scale: 1024.000000,
        offset: 0.000000,
        units: "",
        bits: 12,
        accumulate: true,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "event_timestamp_12",
        field_type: FieldDataType::Byte,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "hr",
        global_message_number: MesgNum::Hr,
        fields,
    }
}
/// Value from 1 to 100 calculated by FirstBeat
pub fn stress_level_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "stress_level_value",
        field_type: FieldDataType::SInt16,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // Time stress score was calculated
    let field = FieldInfo {
        name: "stress_level_time",
        field_type: FieldDataType::DateTime,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    MessageInfo {
        name: "stress_level",
        global_message_number: MesgNum::StressLevel,
        fields,
    }
}
pub fn memo_glob_message() -> MessageInfo {
    let mut fields = HashMap::new();
    // Block of utf8 bytes
    let field = FieldInfo {
        name: "memo",
        field_type: FieldDataType::Byte,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // Allows relating glob to another mesg  If used only required for first part of each memo_glob
    let field = FieldInfo {
        name: "message_number",
        field_type: FieldDataType::UInt16,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // Index of external mesg
    let field = FieldInfo {
        name: "message_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // Sequence number of memo blocks
    let field = FieldInfo {
        name: "part_index",
        field_type: FieldDataType::UInt32,
        def_number: 250,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(250, field);
    MessageInfo {
        name: "memo_glob",
        global_message_number: MesgNum::MemoGlob,
        fields,
    }
}
pub fn ant_channel_id_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "channel_number",
        field_type: FieldDataType::UInt8,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "device_type",
        field_type: FieldDataType::UInt8z,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "device_number",
        field_type: FieldDataType::UInt16z,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "transmission_type",
        field_type: FieldDataType::UInt8z,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "device_index",
        field_type: FieldDataType::DeviceIndex,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    MessageInfo {
        name: "ant_channel_id",
        global_message_number: MesgNum::AntChannelId,
        fields,
    }
}
pub fn ant_rx_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "fractional_timestamp",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 32768.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "mesg_id",
        field_type: FieldDataType::Byte,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "mesg_data",
        field_type: FieldDataType::Byte,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "channel_number",
        field_type: FieldDataType::UInt8,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "data",
        field_type: FieldDataType::Byte,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "ant_rx",
        global_message_number: MesgNum::AntRx,
        fields,
    }
}
pub fn ant_tx_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "fractional_timestamp",
        field_type: FieldDataType::UInt16,
        def_number: 0,
        scale: 32768.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "mesg_id",
        field_type: FieldDataType::Byte,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 8,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "mesg_data",
        field_type: FieldDataType::Byte,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "channel_number",
        field_type: FieldDataType::UInt8,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "data",
        field_type: FieldDataType::Byte,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "ant_tx",
        global_message_number: MesgNum::AntTx,
        fields,
    }
}
pub fn exd_screen_configuration_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "screen_index",
        field_type: FieldDataType::UInt8,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    // number of fields in screen
    let field = FieldInfo {
        name: "field_count",
        field_type: FieldDataType::UInt8,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "layout",
        field_type: FieldDataType::ExdLayout,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "screen_enabled",
        field_type: FieldDataType::Bool,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    MessageInfo {
        name: "exd_screen_configuration",
        global_message_number: MesgNum::ExdScreenConfiguration,
        fields,
    }
}
pub fn exd_data_field_configuration_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "screen_index",
        field_type: FieldDataType::UInt8,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 4,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 4,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "concept_field",
        field_type: FieldDataType::Byte,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "field_id",
        field_type: FieldDataType::UInt8,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "concept_count",
        field_type: FieldDataType::UInt8,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "display_type",
        field_type: FieldDataType::ExdDisplayType,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "title",
        field_type: FieldDataType::String,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    MessageInfo {
        name: "exd_data_field_configuration",
        global_message_number: MesgNum::ExdDataFieldConfiguration,
        fields,
    }
}
pub fn exd_data_concept_configuration_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "screen_index",
        field_type: FieldDataType::UInt8,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let mut components = Vec::new();
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 4,
        accumulate: false,
    };
    components.push(comp_fld);
    let comp_fld = ComponentFieldInfo {
        dest_def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        bits: 4,
        accumulate: false,
    };
    components.push(comp_fld);
    let field = FieldInfo {
        name: "concept_field",
        field_type: FieldDataType::Byte,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: components,
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "field_id",
        field_type: FieldDataType::UInt8,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "concept_index",
        field_type: FieldDataType::UInt8,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "data_page",
        field_type: FieldDataType::UInt8,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "concept_key",
        field_type: FieldDataType::UInt8,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "scaling",
        field_type: FieldDataType::UInt8,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "data_units",
        field_type: FieldDataType::ExdDataUnits,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "qualifier",
        field_type: FieldDataType::ExdQualifiers,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "descriptor",
        field_type: FieldDataType::ExdDescriptors,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "is_signed",
        field_type: FieldDataType::Bool,
        def_number: 11,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    MessageInfo {
        name: "exd_data_concept_configuration",
        global_message_number: MesgNum::ExdDataConceptConfiguration,
        fields,
    }
}
/// Must be logged before developer field is used
pub fn field_description_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "developer_data_index",
        field_type: FieldDataType::UInt8,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "field_definition_number",
        field_type: FieldDataType::UInt8,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "fit_base_type_id",
        field_type: FieldDataType::FitBaseType,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "field_name",
        field_type: FieldDataType::String,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "array",
        field_type: FieldDataType::UInt8,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "components",
        field_type: FieldDataType::String,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "scale",
        field_type: FieldDataType::UInt8,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "offset",
        field_type: FieldDataType::SInt8,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    let field = FieldInfo {
        name: "units",
        field_type: FieldDataType::String,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "bits",
        field_type: FieldDataType::String,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "accumulate",
        field_type: FieldDataType::String,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "fit_base_unit_id",
        field_type: FieldDataType::FitBaseUnit,
        def_number: 13,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(13, field);
    let field = FieldInfo {
        name: "native_mesg_num",
        field_type: FieldDataType::MesgNum,
        def_number: 14,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(14, field);
    let field = FieldInfo {
        name: "native_field_num",
        field_type: FieldDataType::UInt8,
        def_number: 15,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(15, field);
    MessageInfo {
        name: "field_description",
        global_message_number: MesgNum::FieldDescription,
        fields,
    }
}
/// Must be logged before field description
pub fn developer_data_id_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "developer_id",
        field_type: FieldDataType::Byte,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "application_id",
        field_type: FieldDataType::Byte,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "manufacturer_id",
        field_type: FieldDataType::Manufacturer,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "developer_data_index",
        field_type: FieldDataType::UInt8,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "application_version",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    MessageInfo {
        name: "developer_data_id",
        global_message_number: MesgNum::DeveloperDataId,
        fields,
    }
}
pub fn dive_summary_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "reference_mesg",
        field_type: FieldDataType::MesgNum,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "reference_index",
        field_type: FieldDataType::MessageIndex,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    // 0 if above water
    let field = FieldInfo {
        name: "avg_depth",
        field_type: FieldDataType::UInt32,
        def_number: 2,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    // 0 if above water
    let field = FieldInfo {
        name: "max_depth",
        field_type: FieldDataType::UInt32,
        def_number: 3,
        scale: 1000.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    // Time since end of last dive
    let field = FieldInfo {
        name: "surface_interval",
        field_type: FieldDataType::UInt32,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "start_cns",
        field_type: FieldDataType::UInt8,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "end_cns",
        field_type: FieldDataType::UInt8,
        def_number: 6,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(6, field);
    let field = FieldInfo {
        name: "start_n2",
        field_type: FieldDataType::UInt16,
        def_number: 7,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(7, field);
    let field = FieldInfo {
        name: "end_n2",
        field_type: FieldDataType::UInt16,
        def_number: 8,
        scale: 1.000000,
        offset: 0.000000,
        units: "percent",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(8, field);
    let field = FieldInfo {
        name: "o2_toxicity",
        field_type: FieldDataType::UInt16,
        def_number: 9,
        scale: 1.000000,
        offset: 0.000000,
        units: "OTUs",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(9, field);
    let field = FieldInfo {
        name: "dive_number",
        field_type: FieldDataType::UInt32,
        def_number: 10,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(10, field);
    let field = FieldInfo {
        name: "bottom_time",
        field_type: FieldDataType::UInt32,
        def_number: 11,
        scale: 1000.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(11, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "dive_summary",
        global_message_number: MesgNum::DiveSummary,
        fields,
    }
}
pub fn climb_pro_message() -> MessageInfo {
    let mut fields = HashMap::new();
    let field = FieldInfo {
        name: "position_lat",
        field_type: FieldDataType::SInt32,
        def_number: 0,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(0, field);
    let field = FieldInfo {
        name: "position_long",
        field_type: FieldDataType::SInt32,
        def_number: 1,
        scale: 1.000000,
        offset: 0.000000,
        units: "semicircles",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(1, field);
    let field = FieldInfo {
        name: "climb_pro_event",
        field_type: FieldDataType::ClimbProEvent,
        def_number: 2,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(2, field);
    let field = FieldInfo {
        name: "climb_number",
        field_type: FieldDataType::UInt16,
        def_number: 3,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(3, field);
    let field = FieldInfo {
        name: "climb_category",
        field_type: FieldDataType::UInt8,
        def_number: 4,
        scale: 1.000000,
        offset: 0.000000,
        units: "",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(4, field);
    let field = FieldInfo {
        name: "current_dist",
        field_type: FieldDataType::Float32,
        def_number: 5,
        scale: 1.000000,
        offset: 0.000000,
        units: "m",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(5, field);
    let field = FieldInfo {
        name: "timestamp",
        field_type: FieldDataType::DateTime,
        def_number: 253,
        scale: 1.000000,
        offset: 0.000000,
        units: "s",
        accumulate: false,
        subfields: Vec::new(),
        components: Vec::new(),
    };
    fields.insert(253, field);
    MessageInfo {
        name: "climb_pro",
        global_message_number: MesgNum::ClimbPro,
        fields,
    }
}
fn unknown_message(global_message_number: MesgNum) -> MessageInfo {
    MessageInfo {
        name: "unknown",
        global_message_number,
        fields: HashMap::new(),
    }
}
impl MesgNum {
    pub fn message_info(self) -> MessageInfo {
        match self {
            MesgNum::FileId => file_id_message(),
            MesgNum::FileCreator => file_creator_message(),
            MesgNum::TimestampCorrelation => timestamp_correlation_message(),
            MesgNum::Software => software_message(),
            MesgNum::SlaveDevice => slave_device_message(),
            MesgNum::Capabilities => capabilities_message(),
            MesgNum::FileCapabilities => file_capabilities_message(),
            MesgNum::MesgCapabilities => mesg_capabilities_message(),
            MesgNum::FieldCapabilities => field_capabilities_message(),
            MesgNum::DeviceSettings => device_settings_message(),
            MesgNum::UserProfile => user_profile_message(),
            MesgNum::HrmProfile => hrm_profile_message(),
            MesgNum::SdmProfile => sdm_profile_message(),
            MesgNum::BikeProfile => bike_profile_message(),
            MesgNum::Connectivity => connectivity_message(),
            MesgNum::WatchfaceSettings => watchface_settings_message(),
            MesgNum::OhrSettings => ohr_settings_message(),
            MesgNum::ZonesTarget => zones_target_message(),
            MesgNum::Sport => sport_message(),
            MesgNum::HrZone => hr_zone_message(),
            MesgNum::SpeedZone => speed_zone_message(),
            MesgNum::CadenceZone => cadence_zone_message(),
            MesgNum::PowerZone => power_zone_message(),
            MesgNum::MetZone => met_zone_message(),
            MesgNum::DiveSettings => dive_settings_message(),
            MesgNum::DiveAlarm => dive_alarm_message(),
            MesgNum::DiveGas => dive_gas_message(),
            MesgNum::Goal => goal_message(),
            MesgNum::Activity => activity_message(),
            MesgNum::Session => session_message(),
            MesgNum::Lap => lap_message(),
            MesgNum::Length => length_message(),
            MesgNum::Record => record_message(),
            MesgNum::Event => event_message(),
            MesgNum::DeviceInfo => device_info_message(),
            MesgNum::TrainingFile => training_file_message(),
            MesgNum::Hrv => hrv_message(),
            MesgNum::WeatherConditions => weather_conditions_message(),
            MesgNum::WeatherAlert => weather_alert_message(),
            MesgNum::GpsMetadata => gps_metadata_message(),
            MesgNum::CameraEvent => camera_event_message(),
            MesgNum::GyroscopeData => gyroscope_data_message(),
            MesgNum::AccelerometerData => accelerometer_data_message(),
            MesgNum::MagnetometerData => magnetometer_data_message(),
            MesgNum::BarometerData => barometer_data_message(),
            MesgNum::ThreeDSensorCalibration => three_d_sensor_calibration_message(),
            MesgNum::OneDSensorCalibration => one_d_sensor_calibration_message(),
            MesgNum::VideoFrame => video_frame_message(),
            MesgNum::ObdiiData => obdii_data_message(),
            MesgNum::NmeaSentence => nmea_sentence_message(),
            MesgNum::AviationAttitude => aviation_attitude_message(),
            MesgNum::Video => video_message(),
            MesgNum::VideoTitle => video_title_message(),
            MesgNum::VideoDescription => video_description_message(),
            MesgNum::VideoClip => video_clip_message(),
            MesgNum::Set => set_message(),
            MesgNum::Jump => jump_message(),
            MesgNum::Course => course_message(),
            MesgNum::CoursePoint => course_point_message(),
            MesgNum::SegmentId => segment_id_message(),
            MesgNum::SegmentLeaderboardEntry => segment_leaderboard_entry_message(),
            MesgNum::SegmentPoint => segment_point_message(),
            MesgNum::SegmentLap => segment_lap_message(),
            MesgNum::SegmentFile => segment_file_message(),
            MesgNum::Workout => workout_message(),
            MesgNum::WorkoutSession => workout_session_message(),
            MesgNum::WorkoutStep => workout_step_message(),
            MesgNum::ExerciseTitle => exercise_title_message(),
            MesgNum::Schedule => schedule_message(),
            MesgNum::Totals => totals_message(),
            MesgNum::WeightScale => weight_scale_message(),
            MesgNum::BloodPressure => blood_pressure_message(),
            MesgNum::MonitoringInfo => monitoring_info_message(),
            MesgNum::Monitoring => monitoring_message(),
            MesgNum::Hr => hr_message(),
            MesgNum::StressLevel => stress_level_message(),
            MesgNum::MemoGlob => memo_glob_message(),
            MesgNum::AntChannelId => ant_channel_id_message(),
            MesgNum::AntRx => ant_rx_message(),
            MesgNum::AntTx => ant_tx_message(),
            MesgNum::ExdScreenConfiguration => exd_screen_configuration_message(),
            MesgNum::ExdDataFieldConfiguration => exd_data_field_configuration_message(),
            MesgNum::ExdDataConceptConfiguration => exd_data_concept_configuration_message(),
            MesgNum::FieldDescription => field_description_message(),
            MesgNum::DeveloperDataId => developer_data_id_message(),
            MesgNum::DiveSummary => dive_summary_message(),
            MesgNum::ClimbPro => climb_pro_message(),
            _ => unknown_message(self),
        }
    }
}
