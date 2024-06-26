use std::process::Command;

use super::types::{PactlClient, PactlInput, PactlJSONInput};

pub fn get_sink_inputs() -> Vec<PactlInput> {
  let inputs = Command::new("pactl")
    .arg("--format=json")
    .arg("list")
    .arg("short")
    .arg("sink-inputs")
    .output()
    .unwrap();
  let inputsjson: Vec<PactlJSONInput> = serde_json::from_slice(&inputs.stdout).unwrap();
  inputsjson.iter().map(|i| PactlInput { index: i.index.to_string(), sink: i.sink, client: i.client.clone() }).collect()
}

pub fn get_client_matches(app: &str) -> Vec<PactlClient> {
  let clients = get_clients();
  let client_match = clients.iter().filter(|c| c.application_process_binary.contains(app));
  client_match.map(|c| c.clone()).collect()
}

pub fn get_clients() -> Vec<PactlClient> {
  let paclients = Command::new("pactl")
    .arg("--format=json")
    .arg("list")
    .arg("short")
    .arg("clients")
    .output()
    .unwrap();
  serde_json::from_slice(&paclients.stdout).unwrap()
}
