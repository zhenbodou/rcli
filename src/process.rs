use csv::Reader;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Player {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Position")]
    pub position: String,
    #[serde(rename = "DOB")]
    pub dob: String,
    #[serde(rename = "Nationality")]
    pub nationality: String,
    #[serde(rename = "Kit Number")]
    pub kit_number: String,
}

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut players = Vec::new();
    for result in reader.deserialize() {
        let player: Player = result?;
        players.push(player);
    }
    let json_output = serde_json::to_string_pretty(&players)?;
    std::fs::write(output, json_output)?;
    Ok(())
}
