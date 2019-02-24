extern crate chrono;
extern crate csv;
extern crate serde_derive;

mod lib;

use lib::{MessageType, SbsHeader, SbsMessageExtension};
use std::error::Error;
use std::process;

fn read() -> Result<(), Box<Error>> {
    let csv = "SEL,,496,2286,4CA4E5,27215,2010/02/19,18:06:07.710,2010/02/19,18:06:07.710,RYR1427
MSG,4,5,211,4CA2D6,10057,2008/11/28,14:53:49.986,2008/11/28,14:58:51.153,,,408.3,146.4,,,64,,,,,
SEL,,499,2286,4CA4E5,27215,2010/02/19,18:06:07.710,2010/02/19,18:06:07.710,RYR1427";

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(csv.as_bytes());

    for record in reader.records() {
        let rec = record?;
        let frame: SbsHeader = rec.deserialize(None)?;
        println!("- {:?}", frame);

        match frame.message_type {
            MessageType::MSG => {
                let msg: SbsMessageExtension = rec.deserialize(None)?;
                println!("- {:?}", msg);
            }
            _ => (),
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = read() {
        println!("Error reading SBS data: {}", err);
        process::exit(1);
    }
}
