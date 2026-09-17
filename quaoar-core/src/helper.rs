pub fn parse_str(bytes: &[u8]) -> &str{
    str::from_utf8(bytes).unwrap()
}