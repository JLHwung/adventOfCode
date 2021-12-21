macro_rules! DATA_PATH {
    () => {
        "../data/day21.txt"
    };
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

const PAWN_MAX: usize = 10;
const DIE_MAX_P1: usize = 100;

type Input = [usize; 2];

fn process(raw: &str) -> Input {
    let mut lines = raw.lines();
    // normalize pawn to 0-based
    let player0 = lines.next().unwrap().chars().last().unwrap() as usize - b'0' as usize - 1;
    let player1 = lines.next().unwrap().chars().last().unwrap() as usize - b'0' as usize - 1;
    [player0, player1]
}

#[inline]
fn next_pawn(pawn: usize, rhs: usize) -> usize {
    (pawn + PAWN_MAX as usize + rhs) % PAWN_MAX as usize
}

fn p1(input: &Input) -> usize {
    let (mut die_count, mut next_die) = (0, 0);
    let mut score = [0, 0];
    let mut pawn = *input;
    let mut player = 0; // Player 0 plays first
    while usize::max(score[0], score[1]) < 1000 {
        die_count += 3;
        let pawn_step = (3 * next_die + 3) % DIE_MAX_P1 + 3; // die is 0-based
        pawn[player] = next_pawn(pawn[player], pawn_step);
        score[player] += pawn[player] + 1; // pawn is 0-based
        next_die = (next_die + 3) % DIE_MAX_P1;
        player ^= 1;
    }
    die_count * usize::min(score[0], score[1])
}

#[inline]
fn prev_pawn(pawn: usize, rhs: usize) -> usize {
    (pawn + PAWN_MAX - rhs) % PAWN_MAX
}

const WIN: usize = 21;

const DIE_3_FREQUENCY: [usize; 10] = [0, 0, 0, 1, 3, 6, 7, 6, 3, 1];

type Cache = Vec<Vec<[[[usize; 2]; PAWN_MAX]; PAWN_MAX]>>;

/// Problem (score_0, score_1, pawn_0, pawn_1, last_player)
/// 
/// Count the number of universe when observation is made right after
/// the last_player has played. At this time, the score board gives (score_0, score_1)
/// the pawn of each player is pawn_0, pawn_1
fn problem(
    score_0: usize,
    score_1: usize,
    pawn_0: usize,
    pawn_1: usize,
    last_player: usize,
    cache: &mut Cache,
) -> usize {
    let cached = cache[score_0][score_1][pawn_0][pawn_1][last_player];
    if cached != usize::MAX {
        cached
    } else {
        let mut sum = 0;
        if last_player == 0 {
            #[allow(clippy::needless_range_loop)]
            for die in 3..=9 {
                if score_0 > pawn_0 {
                    sum += DIE_3_FREQUENCY[die]
                        * problem(
                            score_0 - pawn_0 - 1,
                            score_1,
                            prev_pawn(pawn_0, die),
                            pawn_1,
                            1,
                            cache,
                        );
                }
            }
        } else {
            #[allow(clippy::needless_range_loop)]
            for die in 3..=9 {
                if score_1 > pawn_1 {
                    sum += DIE_3_FREQUENCY[die]
                        * problem(
                            score_0,
                            score_1 - pawn_1 - 1,
                            pawn_0,
                            prev_pawn(pawn_1, die),
                            0,
                            cache,
                        );
                }
            }
        }
        cache[score_0][score_1][pawn_0][pawn_1][last_player] = sum;
        sum
    }
}

fn p2(input: &Input) -> usize {
    // cache (score0, score_1, p_0, p_1, player_id: 0 | 1)
    let mut cache = vec![vec![[[[usize::MAX; 2]; PAWN_MAX]; PAWN_MAX]; WIN]; WIN];
    // initial conditions:
    cache[0][0] = [[[0; 2]; PAWN_MAX]; PAWN_MAX];
    cache[0][0][input[0]][input[1]][1] = 1;
    let mut winner_count = [0, 0];
    for (winner, count) in winner_count.iter_mut().enumerate() {
        #[allow(clippy::needless_range_loop)]
        for die in 3..=9 {
            for score_winner in WIN..WIN + PAWN_MAX {
                for score_adversary in 0..WIN {
                    for last_pawn_winner in score_winner-WIN..PAWN_MAX {
                        for pawn_adversary in 0..PAWN_MAX {
                            let universe_count = if winner == 0 {
                                problem(
                                    score_winner - last_pawn_winner - 1,
                                    score_adversary,
                                    prev_pawn(last_pawn_winner, die),
                                    pawn_adversary,
                                    1,
                                    &mut cache,
                                )
                            } else {
                                problem(
                                    score_adversary,
                                    score_winner - last_pawn_winner - 1,
                                    pawn_adversary,
                                    prev_pawn(last_pawn_winner, die),
                                    0,
                                    &mut cache,
                                )
                            };
                            *count += DIE_3_FREQUENCY[die] * universe_count;
                        }
                    }
                }
            }
        }
    }
    winner_count.into_iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 925605);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 486638407378784);
    }
}
