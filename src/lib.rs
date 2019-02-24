use chrono::prelude::NaiveDate;
use serde::Deserialize;

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
pub enum MessageType {
    SEL,
    ID,
    AIR,
    STA,
    CLK,
    MSG,
}

#[derive(Debug, Deserialize)]
pub struct SbsHeader {
    pub message_type: MessageType,
    pub transmission_type: Option<u32>,
    pub session_id: u32,
    pub aircraft_id: u32,
    pub hex_ident: String,
    pub flight_id: u32,
    #[serde(with = "read_date")]
    pub generated_date: NaiveDate,
    pub generated_time: String,
    #[serde(with = "read_date")]
    pub logged_date: NaiveDate,
    pub logged_time: String,
    pub callsign: String,
}

#[derive(Debug, Deserialize)]
pub struct SbsMessageExtension {
    pub header: SbsHeader,
    pub altitude: Option<String>,
    pub ground_speed: Option<f64>,
    pub track: Option<f64>,
    pub lat: Option<String>,
    pub lon: Option<String>,
    pub vertical_rate: Option<u32>,
    pub squawk: Option<String>,
    pub alert: Option<bool>,
    pub emergency: Option<bool>,
    pub spi: Option<bool>,
    pub is_on_ground: Option<bool>,
}
