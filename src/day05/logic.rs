use crate::day05::models::Kitchen;

pub fn prepare(kitchen: &mut Kitchen) {
    // First sort the ranges
    kitchen.fresh_ranges.sort();

    // Merge them
    let mut last_range: Option<(i64, i64)> = None;
    let mut fresh_ranges = vec![];
    for range in kitchen.fresh_ranges.iter() {
        if let Some(last_range_inner) = last_range {
            if last_range_inner.1 + 1 >= range.0 {
                last_range = Some((last_range_inner.0, last_range_inner.1.max(range.1)));
            } else {
                fresh_ranges.push(last_range_inner);
                last_range = Some(*range);
            }
        } else {
            last_range = Some(*range);
        }
    }
    if let Some(last_range_inner) = last_range {
        fresh_ranges.push(last_range_inner);
    }
    kitchen.fresh_ranges = fresh_ranges;
}

pub fn solve_part_one(kitchen: &Kitchen) -> usize {
    // For each id, look if it is present in a range
    kitchen
        .ingredients
        .iter()
        .filter(|ingredient| is_in_range(&kitchen.fresh_ranges, **ingredient))
        .count()
}

/// Look if an ingredient is present in a range, to do that, we basically perform a custom
/// binary search in our sorted range
fn is_in_range(fresh_ranges: &[(i64, i64)], ingredient: i64) -> bool {
    let mut low = 0;
    let mut high = fresh_ranges.len() - 1;

    while low <= high {
        let mid = low + (high - low) / 2;
        let (start, end) = fresh_ranges[mid];

        // Return true if the ingredient is in the range
        if start <= ingredient && ingredient <= end {
            return true;
        }

        if end < ingredient {
            low = mid + 1;
        } else if mid == 0 {
            return false;
        } else {
            high = mid - 1;
        }
    }

    false
}

pub fn solve_part_two(kitchen: &Kitchen) -> i64 {
    kitchen.fresh_ranges.iter().map(|(start, end)| end - start + 1).sum()
}
