pub fn parse_address(address: String) -> String {
    address
        .trim_start_matches("0x")
        .chars()
        .take(6)
        .collect::<String>()
        .to_uppercase()
}
