pub fn parse() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let servers: Vec<String> = std::fs::read_to_string("server_list.txt")?
        .lines()
        .map(|s| s.to_string())
        .collect();
    Ok(servers)
}
