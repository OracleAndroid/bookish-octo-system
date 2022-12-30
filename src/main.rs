use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_json;

struct ApiAuth {
    clientid: String,
    clientsecret: String,
    tokenurl: String,
    clienturl: String,
}

fn main(){
    read_config();
}
// Open and read JSON Config file

fn read_config() -> Result<()> {
    let config = File::open("./configuration/clientInfo.json")
        .expect("Cannot open Configuration file");
    Ok(())
    }

