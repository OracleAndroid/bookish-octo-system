use std::fs::File;
use std::io::{prelude::*, Result};

pub mod call_api;


fn read_config_file() -> std::io::Result<()> {
	let mut config_file = File::open("/configuration/appSettings.json")?;
	let mut cfg_file_cont = String::new();
	config_file.read_to_string(&mut cfg_file_cont)?;
    let json: serde_json::Value = serde_json::from_str(&cfg_file_cont).expect("JSON was not well-formatted");
	println!("{:#?}", json);

	Ok(())
}

fn main() {
	read_config_file();
	//call_api::call_api();
}
