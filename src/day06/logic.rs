use itertools::izip;
use nom::character::complete::{i64 as i64_parser, line_ending, one_of, space0, space1};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, terminated};
use nom::{IResult, Parser};

fn parse_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    delimited(space0, separated_list1(space1, i64_parser), (space0, line_ending)).parse(input)
}

fn parse_signs(input: &str) -> IResult<&str, Vec<char>> {
    terminated(separated_list1(space1, one_of("+*")), (space0, line_ending)).parse(input)
}

pub fn parse_input_part_one(input: &str) -> (Vec<Vec<i64>>, Vec<char>) {
    let (res, data) = (many1(parse_numbers), parse_signs).parse(input).unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    data
}

pub fn solve_part_one(input: &str) -> i64 {
    let (numbers, operators) = parse_input_part_one(input);

    if numbers.len() == 3 {
        izip!(numbers[0].iter(), numbers[1].iter(), numbers[2].iter(), operators)
            .map(|(a, b, c, sign)| if sign == '*' { a * b * c } else { a + b + c })
            .sum()
    } else if numbers.len() == 4 {
        izip!(
            numbers[0].iter(),
            numbers[1].iter(),
            numbers[2].iter(),
            numbers[3].iter(),
            operators
        )
        .map(|(a, b, c, d, sign)| if sign == '*' { a * b * c * d } else { a + b + c + d })
        .sum()
    } else {
        unreachable!()
    }
}

fn parse_first_line(input: &str) -> IResult<&str, (i64, char)> {
    map(
        (space0, i64_parser, space0, one_of("*+"), line_ending),
        |(_, value, _, sign, _)| (value, sign),
    )
    .parse(input)
}

fn parse_other_line(input: &str) -> IResult<&str, i64> {
    delimited(space0, i64_parser, (space0, line_ending)).parse(input)
}

fn parse_problem(input: &str) -> IResult<&str, (Vec<i64>, char)> {
    map(
        (parse_first_line, many1(parse_other_line)),
        |((val, sign), mut vals)| {
            vals.push(val);
            (vals, sign)
        },
    )
    .parse(input)
}

pub fn parse_input_part_two(input: &str) -> Vec<(Vec<i64>, char)> {
    let (res, data) = separated_list1((space1, line_ending), parse_problem)
        .parse(input)
        .unwrap();
    if !res.is_empty() {
        panic!("Unable to fully parse the input: {}", res);
    }

    data
}

pub fn solve_part_two(data: &str) -> i64 {
    // transpose the string

    let lines: Vec<Vec<_>> = data.lines().map(|line| line.chars().collect()).collect();
    let len = lines[0].len();
    let res: String = (0..len)
        .flat_map(|i| lines.iter().map(move |inner| inner[i]).chain(std::iter::once('\n')))
        .collect();

    // Parse the data
    let problems = parse_input_part_two(&res);

    problems
        .into_iter()
        .map(|(vals, sign)| {
            vals.into_iter()
                .reduce(|acc, val| if sign == '*' { acc * val } else { acc + val })
                .unwrap()
        })
        .sum()
}
