pub fn parse_from_cmd(text: &String) -> Vec<&str> {
    text.lines().collect::<Vec<_>>()
}
