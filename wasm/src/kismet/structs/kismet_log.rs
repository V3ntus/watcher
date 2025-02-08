use serde::{Deserialize, Serialize};
use crate::kismet::structs::data::Data;
use crate::kismet::structs::data_source::DataSource;
use crate::kismet::structs::devices::Device;
use crate::kismet::structs::message::Message;
use crate::kismet::structs::packets::Packet;
use crate::kismet::structs::snapshots::Snapshot;

#[derive(Serialize, Deserialize)]
pub struct KismetLog {
    pub data: Vec<Data>,
    pub datasources: Vec<DataSource>,
    pub devices: Vec<Device>,
    pub messages: Vec<Message>,
    pub packets: Vec<Packet>,
    pub snapshots: Vec<Snapshot>,
}