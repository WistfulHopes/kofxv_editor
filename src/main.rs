use std::{env, fs::File, io::Write};

use binrw::BinReaderExt;
use cact::CharaActionData;

mod cfile;
mod cact;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let mut file = File::open(path).unwrap();
    let cact: CharaActionData = file.read_le().unwrap();

    let serialized = serde_json::to_string_pretty(&cact)?;
    let new_path = path.to_owned() + ".json";
    let mut output = File::create(new_path)?;
    write!(output, "{}", serialized)
}