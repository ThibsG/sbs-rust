extern crate chrono;
extern crate csv;
#[macro_use]
extern crate serde_derive;

use chrono::prelude::NaiveDate;
use std::error::Error;
use std::process;

mod read_date {
    use chrono::prelude::NaiveDate;
    use serde::{self, Deserialize, Deserializer};
    const FORMAT: &'static str = "%Y/%m/%d";

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Deserialize)]
enum MessageType {
    SEL,
    ID,
    AIR,
    STA,
    CLK,
    MSG,
}

#[derive(Debug, Deserialize)]
struct SbsCommon {
    message_type: MessageType,
    transmission_type: Option<String>,
    session_id: String,
    aircraft_id: String,
    hex_ident: String,
    flight_id: String,
    #[serde(with = "read_date")]
    generated_date: NaiveDate,
    generated_time: String,
    #[serde(with = "read_date")]
    logged_date: NaiveDate,
    logged_time: String,
    callsign: String,
}

#[derive(Debug, Deserialize)]
struct SbsMessage {
    common: SbsCommon,
    altitude: Option<String>,
    ground_speed: Option<String>,
    track: Option<String>,
    lat: Option<String>,
    lon: Option<String>,
    vertical_rate: Option<String>,
    squawk: Option<String>,
    alert: Option<bool>,
    emergency: Option<bool>,
    spi: Option<bool>,
    is_on_ground: Option<bool>,
}

fn read() -> Result<(), Box<Error>> {
    let csv = "SEL,,496,2286,4CA4E5,27215,2010/02/19,18:06:07.710,2010/02/19,18:06:07.710,RYR1427
MSG,4,5,211,4CA2D6,10057,2008/11/28,14:53:49.986,2008/11/28,14:58:51.153,,,408.3,146.4,,,64,,,,,
SEL,,499,2286,4CA4E5,27215,2010/02/19,18:06:07.710,2010/02/19,18:06:07.710,RYR1427";

    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_reader(csv.as_bytes());

    for record in reader.records() {
        println!("----------");
        let frame = record?;
        println!("{:?}", frame);

        if &frame[0] == "MSG" {
            println!("MSG !");
            let msg: SbsMessage = frame.deserialize(None)?;
            println!("{:?}", msg);
        } else {
            println!("NOT A MSG !");
            let cmn: SbsCommon = frame.deserialize(None)?;
            println!("{:?}", cmn);
        }

        /*println!(
            "{} | {:?} | {} | {}",
            frame.message_type, frame.transmission_type, frame.session_id, frame.aircraft_id
        );

        match frame.message_type.as_ref() {
            "MSG" => println!("MSG !"),
            _ => println!("Nan: {}", frame.message_type),
        }*/
    }

    Ok(())
}

fn main() {
    if let Err(err) = read() {
        println!("Error reading SBS data: {}", err);
        process::exit(1);
    }
}
