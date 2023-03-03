use binrw::BinRead;
use serde::{Serialize, Deserialize};

#[derive(BinRead, Serialize, Deserialize)]
pub(crate) struct FileHeader {
    file_identifier: [u8; 4],
    ui_format_version: i32,
}