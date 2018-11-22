pub fn solve(input: Vec<String>) -> String {
    let mut output: Vec<String> = Vec::new();
    let input = convert(input);
    for (first, second) in input {
        let distance = {
            if first == second {
                0
            } else {
                let normalized_first = normalize_to_identity(&first, &second);
                let normalized_second = normalize_to_identity(&second, &first);
                std::cmp::min(
                    get_distance_from_identity(normalized_first),
                    get_distance_from_identity(normalized_second),
                )
            }
        };
        output.push(distance.to_string());
    }
    output.join(" ")
}

use std::str::FromStr;

fn convert(input: Vec<String>) -> Vec<(Vec<u8>, Vec<u8>)> {
    input
        .split(|line| line.is_empty())
        .map(|pair| {
            (
                pair[0]
                    .split_whitespace()
                    .map(|s| u8::from_str(s).unwrap())
                    .collect::<Vec<_>>(),
                pair[1]
                    .split_whitespace()
                    .map(|s| u8::from_str(s).unwrap())
                    .collect::<Vec<_>>(),
            )
        }).collect::<Vec<_>>()
}

fn normalize_to_identity(first: &[u8], second: &[u8]) -> Vec<u8> {
    let mut normalized = Vec::with_capacity(first.len() + 2);
    normalized.push(0);
    normalized.extend(
        second
            .into_iter()
            .filter_map(|&d2| first.iter().position(|&d1| d1 == d2))
            .map(|d| (d + 1) as u8),
    );
    normalized.push((first.len() + 1) as u8);
    normalized
}

fn get_distance_from_identity(starting_permutation: Vec<u8>) -> u32 {
    let mut distance = 0;
    let max_distance = starting_permutation.len() as u32;
    let mut permutations = vec![starting_permutation];
    let mut min_num_breakpoints = get_breakpoints(&permutations[0]).len();
    while distance < max_distance {
        distance += 1;
        let mut new_permutations = vec![];
        for permutation in permutations {
            let breakpoints = get_breakpoints(&permutation);
            for i in 0..breakpoints.len() {
                for j in i + 1..breakpoints.len() {
                    if breakpoints[j] - breakpoints[i] > 1 {
                        let new_permutation =
                            reverse_strip(&permutation, breakpoints[i], breakpoints[j]);
                        let new_breakpoint_count = get_breakpoints(&new_permutation).len();
                        if new_breakpoint_count == 0 {
                            return distance;
                        } else if new_breakpoint_count < min_num_breakpoints {
                            min_num_breakpoints = new_breakpoint_count;
                            new_permutations.clear();
                            new_permutations.push(new_permutation);
                        } else if new_breakpoint_count == min_num_breakpoints {
                            new_permutations.push(new_permutation);
                        }
                    }
                }
            }
        }
        permutations = new_permutations;
    }
    distance
}

fn get_breakpoints(permutation: &[u8]) -> Vec<usize> {
    let mut breakpoints = Vec::new();
    for i in 1..permutation.len() {
        if i8::abs((permutation[i] as i8) - (permutation[i - 1] as i8)) > 1 {
            breakpoints.push(i);
        }
    }
    breakpoints
}

fn reverse_strip(permutation: &[u8], i: usize, j: usize) -> Vec<u8> {
    let mut reversed = permutation.to_vec();
    &reversed[i..j].reverse();
    reversed
}

#[cfg(test)]
mod tests {
    use rear::*;

    #[test]
    fn it_works() {
        let input = r"1 2 3 4 5 6 7 8 9 10
3 1 5 2 7 4 9 6 10 8

3 10 8 2 5 4 7 1 6 9
5 2 3 1 7 4 10 8 6 9

8 6 7 9 4 1 3 10 2 5
8 2 7 6 9 1 5 3 10 4

3 9 10 4 1 8 6 7 5 2
2 9 8 5 1 7 3 4 6 10

1 2 3 4 5 6 7 8 9 10
1 2 3 4 5 6 7 8 9 10"
            .lines()
            .map(String::from)
            .collect();
        assert_eq!(solve(input), String::from("9 4 5 7 0"));
    }
}
