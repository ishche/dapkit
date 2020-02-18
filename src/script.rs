use crate::dap;
use json::object;
use std::io::prelude::*;

#[derive(Debug)]
pub enum Peers {
    Ide,
    Da,
}

#[derive(Debug)]
pub struct ScriptInteraction {
    pub source: Peers,
    pub content: String,
}

#[derive(Debug)]
pub struct DAPScript {
    pub interactions: Vec<ScriptInteraction>,
}

pub fn load_script(filename: &str) -> Result<DAPScript, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    let data = json::parse(&content).unwrap();
    println!("Interaction in {}\n{}", filename, data["interaction"]);
    let mut interaction: Vec<ScriptInteraction> = Vec::new();
    for act in data["interaction"].members() {
        let source: Peers = match act["source"].as_str() {
            Some("ide") => Peers::Ide,
            Some("da") => Peers::Da,
            _ => panic!("source missing"),
        };

        interaction.push(ScriptInteraction {
            source: source,
            content: act["content"].dump(),
        });
    }
    return Ok(DAPScript {
        interactions: interaction,
    });
}

impl DAPScript {
    pub fn run_script<T: Read + Write>(&self, stream: &mut T) {
        // 1. if script says send something - do it now.
        // 2. Wait for message
        // 3. Match message to expected in script
        // 3.1 If no response found - stop
        // 4. goto 1
    
        let msg: dap::DapMessage = dap::read_message(stream).unwrap();
        stream
            .write_all(
                json::stringify(object! {
                    "header" => msg.header,
                    "content" => msg.content
                })
                .as_bytes(),
            )
            .unwrap();
    }
}