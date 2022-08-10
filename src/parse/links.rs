pub fn parse_text(text: &String) -> Vec<&str> {
    text.lines().collect::<Vec<_>>()
}
