use std::{collections::HashSet, io};

use itertools::Itertools;

fn parse_line(x: &str) -> HashSet<usize> {
    // x.split(pat)
    x.split_terminator(['{', ',', '}', ' '])
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap())
        .collect()
}

fn print_set(set: &HashSet<usize>) {
    let mut set_str = set.iter().collect::<Vec<_>>();
    set_str.sort();
    let set_str = set_str.into_iter().map(|x| format!("{x}")).join(", ");
    println!("{{{set_str}}}");
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lines();
    let n = lines.next().unwrap().unwrap().parse::<usize>().unwrap();
    let a_line = lines.next().unwrap().unwrap();
    let b_line = lines.next().unwrap().unwrap();

    let all: HashSet<usize> = HashSet::from_iter(1..=n);
    let a = parse_line(&a_line);
    let b = parse_line(&b_line);

    let a_u_b: HashSet<_> = a.union(&b).copied().collect();
    let a_n_b: HashSet<_> = a.intersection(&b).copied().collect();
    let a_sub_b: HashSet<_> = &a - &b;
    let b_sub_a: HashSet<_> = &b - &a;
    let a_c = &all - &a;
    let b_c = &all - &b;

    print_set(&a_u_b);
    print_set(&a_n_b);
    print_set(&a_sub_b);
    print_set(&b_sub_a);
    print_set(&a_c);
    print_set(&b_c);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parsing() {
        let x = "{1, 2, 3}";
        assert_eq!(parse_line(x), HashSet::from([1, 2, 3]));
    }
}
