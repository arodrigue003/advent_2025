pub fn solve_part_one(data: &[(char, i64)]) -> u64 {
    data.iter().fold((50,0), |(mut pos, mut count), (direction, amount)| {
        pos = match direction {
            'L' => (pos - amount).rem_euclid(100),
            'R' => (pos + amount).rem_euclid(100),
            _ => unreachable!()
        };
        if pos == 0 {
            count += 1;
        };
        (pos, count)
    }).1
}

pub fn solve_part_two(data: &[(char, i64)]) -> i64 {
    data.iter().fold((50,0), |(mut pos, mut count), (direction, amount)| {
         match direction {
            'L' => {
                count += ((100 - pos)%100 + amount) / 100;
                pos = (pos - amount).rem_euclid(100);
            },
            'R' => {
                count += (pos + amount) / 100;
                pos = (pos + amount).rem_euclid(100);
            },
            _ => unreachable!()
        };
        (pos, count)
    }).1
}
