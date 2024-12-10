use day_10::{all_hiking_path_score, count_trail_heads_score};

fn main() {

    let trail_heads_score = count_trail_heads_score("src/resources/puzzle.txt");
    println!("Trail heads score: {}", trail_heads_score);

    let all_hiking_path = all_hiking_path_score("src/resources/puzzle.txt");
    println!("All hiking path score: {}", all_hiking_path);
}
