use day_10::{all_hiking_path_score, count_trail_heads_score};

#[test]
fn should_trail_heads_score() {
    assert_eq!(count_trail_heads_score("tests/resources/puzzle.txt"), 36);
}

#[test]
fn should_give_all_hiking_path_score() {
    assert_eq!(all_hiking_path_score("tests/resources/puzzle.txt"), 81);
}