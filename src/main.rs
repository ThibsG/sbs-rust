extern crate csv;
#[macro_use]
extern crate serde_derive;

use std::process;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct SbsCommon {
    message_type: String,
    transmission_type: Option<String>,
    session_id: String,
    aircraft_id: String,
    hex_ident: String,
    flight_id: String,
    generated_date: String,
    generated_time: String,
    logged_date: String,
    logged_time: String,
    callsign: String
}

fn read() -> Result<(), Box<Error>> {
    let csv = "SEL,,496,2286,4CA4E5,27215,2010/02/19,18:06:07.710,2010/02/19,18:06:07.710,RYR1427";

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv.as_bytes());

    for record in reader.deserialize() {
        let message: SbsCommon = record?;
        println!("{:?}", message);
        println!(
            "{} | {:?} | {} | {}",
            message.message_type,
            message.transmission_type,
            message.session_id,
            message.aircraft_id
        );
    }

    Ok(())
}

fn main() {
    if let Err(err) = read() {
        println!("Error reading SBS data: {}", err);
        process::exit(1);
    }
}