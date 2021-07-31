//! Example of how to read FIT data from a streaming source, which may consist one or more FIT files
use fitparser::{
    de::{FitObject, FitStreamProcessor},
    profile::MesgNum,
};
use std::{
    error::Error,
    io::{self, Read},
};

fn process_obj(processor: &mut FitStreamProcessor, obj: FitObject) -> Result<(), Box<dyn Error>> {
    match obj {
        FitObject::Crc(v) => {
            println!("CRC Value: {}", v)
        }
        FitObject::Header(v) => {
            processor.reset();
            println!(
                "New FIT file, protocol version: {:?}, profile version: {:?}, data length: {}",
                v.protocol_ver_enc(),
                v.profile_ver_enc(),
                v.data_size()
            );
        }
        FitObject::DataMessage(msg) => {
            let record = processor.decode_message(msg)?;
            let kind = record.kind();
            let mut fields: Vec<String> = record
                .into_vec()
                .into_iter()
                .map(|f| {
                    format!(
                        "{}: {}",
                        f.name().to_owned(),
                        fitparser::ValueWithUnits::from(f)
                    )
                })
                .collect();
            fields.sort();
            println!(
                "{} data message (global message number {}): => {}",
                kind,
                kind.as_u16(),
                fields.join(", ")
            );
        }
        FitObject::DefinitionMessage(msg) => {
            println!(
                "definition message {}: {} message (global message number {}) with {} fields",
                msg.local_message_number(),
                MesgNum::from(msg.global_message_number()),
                msg.global_message_number(),
                msg.field_definitions().len() + msg.developer_field_definitions().len()
            );
        }
    }

    Ok(())
}

fn run() -> Result<(), Box<dyn Error>> {
    // Read each FIT data from STDIN and output it
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut data = vec![0; 512]; // messages larger than the buffer size will fail to parse
    let mut processor = FitStreamProcessor::new();
    let mut rem = 0;
    loop {
        let nbytes = handle.read(&mut data[rem..])?;
        let mut buffer = &data[0..nbytes + rem]; // set buffer to be slice of the data actually read
        while !buffer.is_empty() {
            let (buf, obj) = match processor.deserialize_next(buffer) {
                Ok(r) => r,
                Err(e) => match *e {
                    fitparser::ErrorKind::UnexpectedEof(_) => {
                        // break out of loop to read more data
                        break;
                    }
                    _ => return Err(Box::new(e)),
                },
            };
            process_obj(&mut processor, obj)?;
            buffer = buf;
        }
        // shift remaining data to the front of buffer prior to appending more
        rem = buffer.len();
        let tmp = Vec::from(buffer);
        data[..rem].clone_from_slice(&tmp[..rem]);
    }
}

fn main() {
    std::process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}
