use std::collections::HashMap;

use nom::bytes::tag;
use nom::character::complete::{alpha1, line_ending, space1};
use nom::combinator::{map, opt};
use nom::multi::{many1, separated_list1};
use nom::{IResult, Parser};

fn parse_connection(input: &str) -> IResult<&str, (String, Vec<String>)> {
    map(
        (
            alpha1,
            tag(":"),
            space1,
            separated_list1(space1, alpha1),
            opt(line_ending),
        ),
        |(device, _, _, connections, _): (&str, _, _, Vec<&str>, _)| {
            (device.to_string(), connections.into_iter().map(From::from).collect())
        },
    )
    .parse(input)
}
pub fn parse_input(input: String) -> HashMap<String, Vec<String>> {
    let (res, connections) = many1(parse_connection).parse(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    connections.into_iter().collect()
}
