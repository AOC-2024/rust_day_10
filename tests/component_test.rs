use day_10::count_trail_heads_score;

#[test]
fn should_trail_heads_score() {
    assert_eq!(count_trail_heads_score("tests/resources/puzzle.txt"), 36);
}