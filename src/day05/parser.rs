use crate::day05::models::Kitchen;
use nom::bytes::tag;
use nom::character::complete::{i64 as i64_parser, line_ending};
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::{IResult, Parser};

fn parse_range(input: &str) -> IResult<&str, (i64, i64)> {
    map((i64_parser, tag("-"), i64_parser, line_ending), |(start, _, end, _)| {
        (start, end)
    })
    .parse(input)
}

fn parge_ingredient(input: &str) -> IResult<&str, i64> {
    terminated(i64_parser, line_ending).parse(input)
}

fn parse_input_inner(input: &str) -> IResult<&str, (Vec<(i64, i64)>, Vec<i64>)> {
    map(
        (many1(parse_range), many1(line_ending), many1(parge_ingredient)),
        |(ranges, _, ingredients)| (ranges, ingredients),
    )
    .parse(input)
}

pub fn parse_input(input: String) -> Kitchen {
    let (res, (fresh_ranges, ingredients)) = parse_input_inner(&input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    Kitchen {
        fresh_ranges,
        ingredients,
    }
}
