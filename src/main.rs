use std::collections::HashSet;

fn longest_unique_sequence(sequence: &str) -> (usize, usize) {
    let (mut i, mut j) = (0usize, 0usize);
    let mut k = HashSet::<char>::new();
    let (mut bi, mut bj) = (0usize, 0usize);
    while j < sequence.len() {
        let jth_char = sequence.chars().nth(j).unwrap();
        if k.contains(&jth_char) {
            let ith_char = sequence.chars().nth(i).unwrap();
            k.remove(&ith_char);
            i += 1;
        } else {
            k.insert(jth_char);
            j += 1;
        }
        if j - i > bj - bi {
            bi = i;
            bj = j;
        }
    }
    (bi, bj)
}

fn main() {
    let sequence: &str = "Premature optimization is the root of all evil -- DonaldKnuth";
    let (i, j) = longest_unique_sequence(sequence);
    println!("{} {} {}", i, j, &sequence[i..j]);
}
