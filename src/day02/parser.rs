use nom::bytes::tag;
use nom::character::complete::{i64 as i64_parser, line_ending};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::{IResult, Parser};

fn parse_range(input: &str) -> IResult<&str, (i64, i64)> {
    map((i64_parser, tag("-"), i64_parser), |(start, _, end)| (start, end)).parse(input)
}

fn parse_ranges(input: &str) -> IResult<&str, Vec<(i64, i64)>> {
    terminated(separated_list1(tag(","), parse_range), line_ending).parse(input)
}

pub fn parse_input(input: String) -> Vec<(i64, i64)> {
    let (res, ranges) = parse_ranges(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    ranges
}
