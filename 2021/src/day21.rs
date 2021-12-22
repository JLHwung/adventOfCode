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

const WIN: usize = 21;

const DIE_3_MAX: usize = 7;

// 3, 4, 5, 6, 7, 8, 9
const DIE_3_FREQUENCY: [usize; DIE_3_MAX] = [1, 3, 6, 7, 6, 3, 1];

type Cache = Vec<Vec<[[Option<[usize; 2]>; PAWN_MAX]; PAWN_MAX]>>;

/// Problem (score_0, score_1, pawn_0, pawn_1, last_player)
///
/// Count the winning universe of each player, when starting at initial
/// score score_0, score_1, and initial pawn pawn_0, pawn_1
fn problem(
    score_0: usize,
    score_1: usize,
    pawn_0: usize,
    pawn_1: usize,
    cache: &mut Cache,
) -> [usize; 2] {
    if let Some(cached) = cache[score_0][score_1][pawn_0][pawn_1] {
        cached
    } else {
        let mut winner_count = [0; 2];
        for (die, frequency) in DIE_3_FREQUENCY.iter().enumerate() {
            let next_pawn_0 = next_pawn(pawn_0, die + 3);
            let next_score_0 = score_0 + next_pawn_0 + 1;
            if next_score_0 >= WIN {
                winner_count[0] += frequency;
            } else {
                let sub_winner_count = problem(score_1, next_score_0, pawn_1, next_pawn_0, cache);
                winner_count[0] += frequency * sub_winner_count[1];
                winner_count[1] += frequency * sub_winner_count[0];
            }
        }
        cache[score_0][score_1][pawn_0][pawn_1] = Some(winner_count);
        winner_count
    }
}

fn p2(input: &Input) -> usize {
    let mut cache = vec![vec![[[None; PAWN_MAX]; PAWN_MAX]; WIN]; WIN];
    let winner_count = problem(0, 0, input[0], input[1], &mut cache);
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
