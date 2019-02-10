

pub mod sbs1 {
    pub enum MessageType {
        SelChange,
        NewId,
        NewAircraft,
        StatusAircraft,
        Click,
        Transmission
    }

    pub struct Message {
        msg_type: MessageType,
        tr_type: String,
        session_id: String,
    }
}