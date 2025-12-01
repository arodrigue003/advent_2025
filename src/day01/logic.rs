pub fn solve_part_one(data: &[(char, i64)]) -> u64 {
    data.iter()
        .fold((50, 0), |(pos, count), (direction, amount)| match direction {
            'L' => (
                (pos - amount).rem_euclid(100),
                count + ((pos - amount) % 100 == 0) as u64,
            ),
            'R' => (
                (pos + amount).rem_euclid(100),
                count + ((pos + amount) % 100 == 0) as u64,
            ),
            _ => unreachable!(),
        })
        .1
}

pub fn solve_part_two(data: &[(char, i64)]) -> i64 {
    data.iter()
        .fold((50, 0), |(pos, count), (direction, amount)| match direction {
            'L' => (
                (pos - amount).rem_euclid(100),
                count + ((100 - pos) % 100 + amount) / 100,
            ),
            'R' => ((pos + amount).rem_euclid(100), count + (pos + amount) / 100),
            _ => unreachable!(),
        })
        .1
}
