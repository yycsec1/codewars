fn main() {
    assert_eq!(get_count("abracadabra"), 5)
}

fn get_count(string: &str) -> usize {
    string.chars().filter(|&c| "aeiou".contains(c)).count()
}