use std::{env, fs::{File, self}, io::{Write}};

use binrw::BinReaderExt;
use cact::CharaActionData;

mod cfile;
mod cact;

fn parse_cact(path: &String) -> std::io::Result<()> {
    let mut file = File::open(path).unwrap();
    let cact: CharaActionData = file.read_le().unwrap();

    let serialized = serde_json::to_string_pretty(&cact)?;
    let new_path = path.to_owned() + ".json";
    let mut output = File::create(new_path)?;
    write!(output, "{}", serialized)
}

fn rebuild_cact(path: &String) -> std::io::Result<()> {
    let mut cact: CharaActionData = serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    cact.write(&mut buf);

    let new_path: String = path.clone().drain(0..path.len() - 5).collect();
    let mut output = File::create(new_path).unwrap();
    output.write_all(&buf)
}

fn help() {
    println!("Incorrect arguments!");
    println!("The first argument should be -p to parse, or -r to rebuild.");
    println!("If parsing, the second argument should be the path to the .cact file.");
    println!("Extract this from the character's CACT uasset with UAssetGUI.");
    println!("If rebuilding, the second argument should be the path to the .json file.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[2];
    
    match args.len() {
        3 => {
            if args[1] == "-p"
            {
                match parse_cact(path)
                {
                    Ok(()) => println!("Successfully parsed .cact file!"),
                    Err(e) => println!("Failed to parse .cact file! Error: {}", e)
                }
            }
            else if args[1] == "-r"
            {
                match rebuild_cact(path)
                {
                    Ok(()) => println!("Successfully rebuilt .cact file!"),
                    Err(e) => println!("Failed to rebuild .cact file! Error: {}", e)
                }
            }
            else {
                help()
            }
        }
        _ => help()
    }
}