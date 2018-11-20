pub fn solve(input: Vec<String>) -> String {
    let mut output = Vec::new();
    let input = convert(input);
    let num_pairs = input.len() / 2;
    let mut input = input.into_iter();
    for _ in 0..num_pairs {
        let mut first = input.next().unwrap();
        let mut second = input.next().unwrap();
        output.push(get_distance(&mut first, &mut second).to_string());
    }
    output.join(" ")
}

use std::str::FromStr;

fn convert(input: Vec<String>) -> Vec<Vec<u8>> {
    input
        .iter()
        .map(|s| s.split_whitespace())
        .map(|s| s.filter_map(|i| u8::from_str(i).ok()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn get_distance(first: &mut [u8], second: &mut [u8]) -> u32 {
    first.reverse();
    second.reverse();
    0
}

#[cfg(test)]
mod tests {
    use rear::*;

    #[test]
    fn it_works() {
        let input = vec![
            "1 2 3 4 5 6 7 8 9 10",
            "3 1 5 2 7 4 9 6 10 8",
            "3 10 8 2 5 4 7 1 6 9",
            "5 2 3 1 7 4 10 8 6 9",
            "8 6 7 9 4 1 3 10 2 5",
            "8 2 7 6 9 1 5 3 10 4",
            "3 9 10 4 1 8 6 7 5 2",
            "2 9 8 5 1 7 3 4 6 10",
            "1 2 3 4 5 6 7 8 9 10",
            "1 2 3 4 5 6 7 8 9 10",
        ].into_iter()
        .map(String::from)
        .collect();
        assert_eq!(solve(input), String::from("9 4 5 7 0"));
    }
}
