use std::collections::HashMap;

pub fn test() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = [String::from("Blue"), String::from("Yellow")];
    let initial_scores = [10, 50];
    let _scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}
