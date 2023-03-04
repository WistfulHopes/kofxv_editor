use std::{io::Write};

use binrw::{BinRead, BinWrite};
use byteorder::{WriteBytesExt, LE};
use serde::{Serialize, Deserialize};

#[derive(BinRead, BinWrite, Serialize, Deserialize)]
pub(crate) struct FileHeader {
    file_identifier: [u8; 4],
    ui_format_version: i32,
}

impl FileHeader {
    pub fn write(&self, buf: &mut Vec<u8>) {
        buf.write_all(&self.file_identifier[..]).unwrap();
        buf.write_i32::<LE>(self.ui_format_version).unwrap();
    }
}