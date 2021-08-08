use std::collections::VecDeque;

// part 1 and part 2 use the same function
pub fn day9(num_players: usize, last_marble_points: usize) -> usize {
    let mut player_scores = vec![0; num_players];
    let mut ring = VecDeque::from(vec![0, 1]);
    ring.reserve(last_marble_points);

    for i in 2..=last_marble_points {
        if i % 23 == 0{
            ring.rotate_left(7);
            player_scores[i % num_players] += i + ring.pop_back().unwrap();
        } else {
            ring.rotate_right(2);
            ring.push_back(i);
        }
    }

    player_scores.into_iter().max().unwrap()
}
