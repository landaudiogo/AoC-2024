use std::{collections::VecDeque, io};

fn is_diff_abs_valid(val: i64) -> bool {
    val.abs() >= 1 && val.abs() <= 3
}

fn validate_diff_pair(left: i64, right: i64, trend: &mut Option<bool>) -> bool {
    let (sign_left, sign_right) = (left > 0, right > 0);
    if let Some(trend) = trend {
        return (sign_left == sign_right)
            && (sign_left == *trend)
            && is_diff_abs_valid(left)
            && is_diff_abs_valid(right);
    }

    if (sign_left == sign_right) && is_diff_abs_valid(left) && is_diff_abs_valid(right) {
        *trend = Some(sign_left);
        return true;
    }
    false
}

fn fix_pair(first: i64, second: i64, trend: Option<bool>) -> Option<i64> {
    // Drop first
    let res = first + second;
    let trend_component = if let Some(trend) = trend {
        (res > 0) == trend
    } else {
        true
    };
    if is_diff_abs_valid(res) && trend_component {
        return Some(res);
    }

    // Forget Last
    let trend_component = if let Some(trend) = trend {
        (first > 0) == trend
    } else {
        true
    };
    if is_diff_abs_valid(first) && trend_component {
        return Some(res);
    }

    None
}

fn fix_triplet(
    first: i64,
    second: i64,
    third: i64,
    contains_first: bool,
    contains_last: bool,
    trend: &mut Option<bool>,
) -> Option<(i64, i64)> {
    // drop first
    let left = first + second;
    if validate_diff_pair(left, third, trend) {
        return Some((left, third));
    }

    // drop second
    let right = second + third;
    if validate_diff_pair(first, right, trend) {
        return Some((first, right));
    }

    // contains first edge case
    if contains_first && validate_diff_pair(second, third, trend) {
        return Some((second, third));
    }

    // contains last edge case
    if contains_last && validate_diff_pair(first, second, trend) {
        return Some((first, second));
    }

    None
}

fn main() {
    let mut line = String::new();
    let mut sum = 0;

    while let Ok(len) = io::stdin().read_line(&mut line) {
        if len == 0 {
            break;
        }

        let mut levels = line.split_whitespace();
        let mut prev = levels.next().unwrap().parse::<i64>().unwrap();
        let mut diffs: VecDeque<_> = levels
            .into_iter()
            .map(|level| {
                let curr = level.parse::<i64>().unwrap();
                let res = curr - prev;
                prev = curr;
                res
            })
            .collect();

        match diffs.len() {
            0 => {
                sum += 1;
                continue;
            }
            1 => {
                let diff = diffs[0];
                if is_diff_abs_valid(diff) {
                    sum += 1;
                }
                continue;
            }
            _ => {}
        }

        let mut is_first = true;
        let mut trend = None;
        let mut is_valid = true;
        let mut fixed = false;

        while let (Some(first), Some(second)) = (diffs.pop_front(), diffs.pop_front()) {
            if validate_diff_pair(first, second, &mut trend) {
                diffs.push_front(second);
            } else {
                if fixed {
                    is_valid = false;
                    break;
                }

                if let Some(third) = diffs.pop_front() {
                    let pair =
                        fix_triplet(first, second, third, is_first, diffs.len() == 0, &mut trend);
                    if let Some(pair) = pair {
                        diffs.push_front(pair.1)
                    } else {
                        is_valid = false;
                        break;
                    }
                } else {
                    let res = fix_pair(first, second, trend);
                    if res.is_none() {
                        is_valid = false;
                        break;
                    }
                }

                fixed = true;
            }

            is_first = false;
        }

        if is_valid {
            sum += 1;
        }
        line.truncate(0);
    }
    println!("{:?}", sum)
}