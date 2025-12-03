pub fn parse_input(input: String) -> Vec<String> {
    input.lines().map(From::from).collect()
}
