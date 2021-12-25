use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Display, Formatter};
use std::mem::swap;

macro_rules! DATA_PATH {
    () => {
        "../data/day23.txt"
    };
}

const HALLWAY_LEN: usize = 11;
const ROOM_NUM: usize = 4;

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Amphipod {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl Amphipod {
    fn energy(&self) -> usize {
        10usize.pow(*self as u32)
    }

    fn target_room(&self) -> usize {
        *self as usize
    }
}

impl From<u8> for Amphipod {
    fn from(val: u8) -> Self {
        match val {
            0 => Amphipod::A,
            1 => Amphipod::B,
            2 => Amphipod::C,
            3 => Amphipod::D,
            _ => unreachable!(),
        }
    }
}

fn abs_sub(left: usize, right: usize) -> usize {
    if left > right {
        left - right
    } else {
        right - left
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State<const R: usize> {
    hallway: [Option<Amphipod>; HALLWAY_LEN],
    rooms: [[Option<Amphipod>; R]; ROOM_NUM],
}

impl<const R: usize> From<u64> for State<R> {
    fn from(val: u64) -> Self {
        fn decode_space(encoded: u64) -> Option<Amphipod> {
            match encoded {
                0 => None,
                _ => Some(Amphipod::from(encoded as u8 - 1)),
            }
        }

        let mut encoded = val;
        let mut iterator = std::iter::from_fn(move || {
            let result = decode_space(encoded % 5);
            encoded /= 5;
            Some(result)
        });

        Self {
            hallway: [(); HALLWAY_LEN].map(|_| iterator.next().unwrap()),
            rooms: [(); ROOM_NUM].map(|_| [(); R].map(|_| iterator.next().unwrap())),
        }
    }
}

impl<const R: usize> State<R> {
    fn encode(&self) -> u64 {
        fn space_encode(space: &Option<Amphipod>) -> u64 {
            match space {
                Some(amphipod) => amphipod.target_room() as u64 + 1,
                None => 0,
            }
        }
        let mut result: u64 = 0;
        for space in self
            .rooms
            .iter()
            .flatten()
            .rev()
            .chain(self.hallway.iter().rev())
        {
            result = result * 5 + space_encode(space);
        }
        result
    }

    fn goal() -> Self {
        Self {
            hallway: [None; HALLWAY_LEN],
            rooms: [
                [Some(Amphipod::A); R],
                [Some(Amphipod::B); R],
                [Some(Amphipod::C); R],
                [Some(Amphipod::D); R],
            ],
        }
    }

    fn hallway_pos_above_room(&self, room_index: usize) -> usize {
        2 + (room_index) * 2
    }

    fn is_above_room(&self, x: usize) -> bool {
        x >= 2 && (x - 2) % 2 == 0 && ((x - 2) / 2) < ROOM_NUM
    }

    // return the first member that could exit the room
    fn find_exit_room_pos(&self, room_index: usize) -> Option<(usize, Amphipod)> {
        let room = self.rooms[room_index];
        for (pos, status) in room.iter().enumerate() {
            if let Some(amphipod) = status {
                if room[pos..]
                    .iter()
                    .all(|&x| x.unwrap().target_room() == room_index)
                {
                    return None;
                } else {
                    return Some((pos, *amphipod));
                }
            }
        }
        None
    }

    // returns the deepest vacant room position in given room
    fn find_enter_room_pos(&self, room_index: usize) -> Option<usize> {
        let room = self.rooms[room_index];
        for (pos, status) in room.iter().enumerate().rev() {
            if let Some(amphipod) = status {
                if amphipod.target_room() != room_index {
                    return None;
                }
            } else {
                return Some(pos);
            }
        }
        None
    }

    fn is_hallway_clear(&self, start_pos: usize, end_pos: usize) -> bool {
        let range = if start_pos < end_pos {
            (start_pos + 1)..(end_pos + 1)
        } else {
            end_pos..start_pos
        };
        self.hallway[range].iter().all(|x| x.is_none())
    }

    fn hallway_to_room_moves(&self) -> Vec<(Self, usize)> {
        self.hallway
            .iter()
            .enumerate()
            .filter_map(|(h_pos, status)| {
                if let Some(amphipod) = status {
                    let target_room = amphipod.target_room();
                    if let Some(r_pos) = self.find_enter_room_pos(target_room) {
                        let intersection = self.hallway_pos_above_room(target_room);
                        if !self.is_hallway_clear(h_pos, intersection) {
                            return None;
                        }
                        let steps = abs_sub(h_pos, intersection) + r_pos + 1;
                        let cost = amphipod.energy() * steps;
                        let mut state = *self;
                        swap(
                            &mut state.hallway[h_pos],
                            &mut state.rooms[target_room][r_pos],
                        );
                        return Some((state, cost));
                    }
                }
                None
            })
            .collect()
    }

    fn hallway_spaces_around(&self, h_pos: usize) -> impl Iterator<Item = usize> + '_ {
        let left_it = (0..h_pos)
            .rev()
            .take_while(|pos| self.hallway[*pos].is_none());
        let right_it = ((h_pos + 1)..HALLWAY_LEN).take_while(|pos| self.hallway[*pos].is_none());
        left_it.chain(right_it)
    }

    fn room_to_hallway_moves(&self) -> Vec<(Self, usize)> {
        let mut results = vec![];
        for source_room in 0..ROOM_NUM {
            if let Some((r_pos, amphipod)) = self.find_exit_room_pos(source_room) {
                let intersection = self.hallway_pos_above_room(source_room);
                for h_pos in self
                    .hallway_spaces_around(intersection)
                    .filter(|&h_pos| !self.is_above_room(h_pos))
                {
                    let steps = abs_sub(h_pos, intersection) + r_pos + 1;
                    let cost = amphipod.energy() * steps;
                    let mut state = *self;
                    swap(
                        &mut state.hallway[h_pos],
                        &mut state.rooms[source_room][r_pos],
                    );

                    results.push((state, cost));
                }
            }
        }
        results
    }

    fn next_moves(&self) -> Vec<(Self, usize)> {
        let mut results = self.hallway_to_room_moves();
        let results2 = self.room_to_hallway_moves();
        results.extend(results2);
        results
    }

    // estimate cost between this state to goal state. The heuristics
    // should not overestimate the cost otherwise A* may not return
    // optimal results
    fn heuristics(&self) -> usize {
        let mut h_score = 0;
        for (room_index, room) in self.rooms.iter().enumerate() {
            let mut will_be_popped = false;
            let expected_amphipod_energy = 10usize.pow(room_index as u32);
            let pos_above_room = self.hallway_pos_above_room(room_index);
            for (r_pos, &status) in room.iter().enumerate().rev() {
                if let Some(amphipod) = status {
                    let target_room = amphipod.target_room();
                    if target_room != room_index {
                        let hallway_distance =
                            abs_sub(pos_above_room, self.hallway_pos_above_room(target_room));
                        // The amphipod moves from this position to its the pos above its target room
                        h_score += amphipod.energy() * (hallway_distance + r_pos + 1);
                        // The expected amphipod moves from the pos above to this position
                        h_score += expected_amphipod_energy * (r_pos + 1);
                        // All amphipods upper than this position has to be popped out of the room
                        will_be_popped = true;
                    } else if will_be_popped {
                        // The amphipod will be popped out to some pos around room entrance and the pushed back
                        h_score += 2 * expected_amphipod_energy * (r_pos + 1 + 1);
                    }
                } else {
                    // The expected amphipod moves from the pos above to this vacant position
                    h_score += expected_amphipod_energy * (r_pos + 1);
                }
            }
        }
        for (h_pos, status) in self.hallway.iter().enumerate() {
            if let Some(amphipod) = status {
                let target_room = amphipod.target_room();
                let intersection = self.hallway_pos_above_room(target_room);
                // Moves from hallway to the pos above its target room
                h_score += amphipod.energy() * (abs_sub(h_pos, intersection));
            }
        }
        h_score
    }
}

fn main() {
    let raw = include_str!(DATA_PATH!());
    let input = process(raw);
    println!("Answer of p1: {}", p1(&input));
    println!("Answer of p2: {}", p2(&input));
}

type Input = Vec<Amphipod>;

fn process(raw: &str) -> Input {
    raw.chars()
        .filter_map(|ch| match ch {
            'A'..='D' => Some(Amphipod::from(ch as u8 - b'A')),
            _ => None,
        })
        .collect()
}

#[derive(Eq, PartialEq)]
struct QueueItem {
    encoded: u64,
    cost: usize,
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A* Search
fn solve<const R: usize>(initial_state: State<R>) -> usize {
    let mut g_scores = HashMap::from([(initial_state.encode(), 0usize)]);
    let mut heap = BinaryHeap::from(vec![QueueItem {
        encoded: initial_state.encode(),
        cost: 0,
    }]);

    while let Some(QueueItem { encoded, cost }) = heap.pop() {
        let current_dist = g_scores[&encoded];
        let current_state = State::<R>::from(encoded);

        if encoded == State::<R>::goal().encode() {
            return cost;
        }

        for (state, cost) in current_state.next_moves() {
            let new_cost = current_dist + cost;
            let encoded_state = state.encode();
            if new_cost < *g_scores.get(&encoded_state).unwrap_or(&usize::MAX) {
                g_scores.insert(encoded_state, new_cost);
                heap.push(QueueItem {
                    encoded: encoded_state,
                    cost: new_cost + state.heuristics(),
                });
            }
        }
    }
    unreachable!()
}

fn p1(input: &[Amphipod]) -> usize {
    let initial_state = State {
        hallway: [None; HALLWAY_LEN],
        rooms: [
            [Some(input[0]), Some(input[ROOM_NUM])],
            [Some(input[1]), Some(input[1 + ROOM_NUM])],
            [Some(input[2]), Some(input[2 + ROOM_NUM])],
            [Some(input[3]), Some(input[3 + ROOM_NUM])],
        ],
    };
    solve(initial_state)
}

fn p2(input: &[Amphipod]) -> usize {
    // #D#C#B#A#
    // #D#B#A#C#
    let initial_state = State {
        hallway: [None; HALLWAY_LEN],
        rooms: [
            [
                Some(input[0]),
                Some(Amphipod::D),
                Some(Amphipod::D),
                Some(input[ROOM_NUM]),
            ],
            [
                Some(input[1]),
                Some(Amphipod::C),
                Some(Amphipod::B),
                Some(input[1 + ROOM_NUM]),
            ],
            [
                Some(input[2]),
                Some(Amphipod::B),
                Some(Amphipod::A),
                Some(input[2 + ROOM_NUM]),
            ],
            [
                Some(input[3]),
                Some(Amphipod::A),
                Some(Amphipod::C),
                Some(input[3 + ROOM_NUM]),
            ],
        ],
    };
    solve(initial_state)
}

impl<const R: usize> Display for State<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let space_to_str = |space| {
            if let Some(amphipod) = space {
                format!("{:?}", amphipod)
            } else {
                ".".to_string()
            }
        };

        writeln!(f, "{}", "#".repeat(self.hallway.len() + 2))?;
        writeln!(f, "#{}#", self.hallway.map(space_to_str).join(""))?;
        writeln!(
            f,
            "###{}###",
            self.rooms.map(|r| space_to_str(r[0])).join("#")
        )?;
        for r_pos in 1..R {
            writeln!(
                f,
                "  #{}#  ",
                self.rooms.map(|r| space_to_str(r[r_pos])).join("#")
            )?;
        }
        write!(f, "  {}  ", "#".repeat(ROOM_NUM * 2 + 1))?;
        Ok(())
    }
}

#[cfg(test)]
mod test {

    macro_rules! SAMPLE_PATH {
        () => {
            "../data/day23.dbg.txt"
        };
    }

    use super::*;

    fn create_empty_state<const R: usize>() -> State<R> {
        State {
            hallway: [None; HALLWAY_LEN],
            rooms: [[None; R]; ROOM_NUM],
        }
    }

    #[test]
    fn test_encode_decode_roundtrip() {
        let input = process(include_str!(DATA_PATH!()));
        let state = State {
            hallway: [None; HALLWAY_LEN],
            rooms: [
                [
                    Some(input[0]),
                    Some(Amphipod::D),
                    Some(Amphipod::D),
                    Some(input[ROOM_NUM]),
                ],
                [
                    Some(input[1]),
                    Some(Amphipod::C),
                    Some(Amphipod::B),
                    Some(input[1 + ROOM_NUM]),
                ],
                [
                    Some(input[2]),
                    Some(Amphipod::B),
                    Some(Amphipod::A),
                    Some(input[2 + ROOM_NUM]),
                ],
                [
                    Some(input[3]),
                    Some(Amphipod::A),
                    Some(Amphipod::C),
                    Some(input[3 + ROOM_NUM]),
                ],
            ],
        };
        let new_state = State::<4>::from(state.encode());
        assert_eq!(state, new_state);
    }

    #[test]
    fn test_is_hallway_clear() {
        let mut state = create_empty_state::<2>();
        state.hallway[5] = Some(Amphipod::A);
        state.hallway[7] = Some(Amphipod::B);
        assert!(state.is_hallway_clear(5, 4));
        assert_eq!(state.is_hallway_clear(5, 7), false);
    }

    #[test]
    fn test_find_exit_room_pos() {
        let mut state = create_empty_state();
        state.rooms[0] = [Some(Amphipod::A), Some(Amphipod::A)];
        assert_eq!(state.find_exit_room_pos(0), None);

        state = create_empty_state();
        state.rooms[0] = [Some(Amphipod::A), Some(Amphipod::B)];
        assert_eq!(state.find_exit_room_pos(0), Some((0, Amphipod::A)));

        state = create_empty_state();
        state.rooms[0] = [None, Some(Amphipod::B)];
        assert_eq!(state.find_exit_room_pos(0), Some((1, Amphipod::B)));

        state = create_empty_state();
        assert_eq!(state.find_exit_room_pos(0), None);
    }

    #[test]
    fn test_find_enter_room_pos() {
        let mut state = State {
            hallway: [None; HALLWAY_LEN],
            rooms: [[None, Some(Amphipod::B)]; ROOM_NUM],
        };
        assert_eq!(state.find_enter_room_pos(0), None);

        state = State {
            hallway: [None; HALLWAY_LEN],
            rooms: [[None, Some(Amphipod::A)]; ROOM_NUM],
        };
        assert_eq!(state.find_enter_room_pos(0), Some(0));
    }

    #[test]
    fn test_room_to_hallway_moves_top() {
        let mut state = create_empty_state();
        state.rooms[1] = [Some(Amphipod::B), Some(Amphipod::A)];
        state.hallway[1] = Some(Amphipod::A);
        state.hallway[5] = Some(Amphipod::B);
        let moves = state.room_to_hallway_moves();
        assert_eq!(moves.len(), 1);
        let (state, cost) = moves[0];
        assert_eq!(cost, 20);
        assert_eq!(state.rooms[1][0], None);
        assert_eq!(state.hallway[3], Some(Amphipod::B));
    }

    #[test]
    fn test_room_to_hallway_moves_lower() {
        let mut state = create_empty_state();
        state.rooms[0] = [None, Some(Amphipod::B)];
        state.hallway[3] = Some(Amphipod::A);
        let moves = state.room_to_hallway_moves();
        assert_eq!(moves.len(), 2);
        let (state, cost) = moves[0];
        assert_eq!(cost, 30);
        assert_eq!(state.rooms[0][0], None);
        assert_eq!(state.hallway[1], Some(Amphipod::B));
        assert_eq!(moves[1].1, 40);
    }

    #[test]
    fn hallway_to_room_moves_empty() {
        let mut state = create_empty_state::<2>();
        state.hallway[3] = Some(Amphipod::B);
        state.hallway[5] = Some(Amphipod::A);
        let moves = state.hallway_to_room_moves();
        assert_eq!(moves.len(), 1);
        let (state, cost) = moves[0];
        assert_eq!(cost, 30);
        assert_eq!(state.rooms[1][1], Some(Amphipod::B));
        assert_eq!(state.hallway[3], None);
        assert_eq!(state.hallway[5], Some(Amphipod::A));
    }

    #[test]
    fn hallway_to_room_moves_two_hallway() {
        let mut state = create_empty_state::<2>();
        state.rooms[0] = [Some(Amphipod::B), Some(Amphipod::A)];
        state.rooms[1] = [None, Some(Amphipod::D)];
        state.rooms[2] = [None, Some(Amphipod::C)];
        state.rooms[3] = [Some(Amphipod::D), Some(Amphipod::A)];
        state.hallway[3] = Some(Amphipod::C);
        state.hallway[7] = Some(Amphipod::B);
        let moves = state.hallway_to_room_moves();
        assert_eq!(moves.len(), 1);
        let (_, cost) = moves[0];
        assert_eq!(cost, 400);
    }

    #[test]
    fn hallway_to_room_moves_non_empty() {
        let mut state = create_empty_state();
        state.rooms[1] = [None, Some(Amphipod::B)];
        state.hallway[3] = Some(Amphipod::B);
        let moves = state.hallway_to_room_moves();
        assert_eq!(moves.len(), 1);
        let (state, cost) = moves[0];
        assert_eq!(cost, 20);
        assert_eq!(state.rooms[1][0], Some(Amphipod::B));
        assert_eq!(state.hallway[3], None);
    }

    #[test]
    fn hallway_to_room_moves_other_amphipods_only() {
        let mut state = create_empty_state();
        state.rooms[1] = [None, Some(Amphipod::A)];
        state.hallway[3] = Some(Amphipod::B);
        let moves = state.hallway_to_room_moves();
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn hallway_to_room_moves_other_amphipods_below() {
        let mut state = create_empty_state();
        state.rooms[1] = [Some(Amphipod::B), Some(Amphipod::A)];
        state.hallway[3] = Some(Amphipod::B);
        let moves = state.hallway_to_room_moves();
        assert_eq!(moves.len(), 0);
    }

    #[test]
    fn test_heuristics_2() {
        let mut state = State::<2>::goal();
        state.rooms[1][0] = None;
        state.hallway[3] = Some(Amphipod::B);
        assert_eq!(state.heuristics(), Amphipod::B.energy() * 2);

        let mut state = State::<2>::goal();
        state.rooms[0][0] = Some(Amphipod::B);
        assert_eq!(
            state.heuristics(),
            Amphipod::B.energy() * 3 + Amphipod::A.energy() * 1
        );

        let mut state = State::<2>::goal();
        state.rooms[0][0] = Some(Amphipod::B);
        state.rooms[1][0] = Some(Amphipod::A);
        assert_eq!(
            state.heuristics(),
            Amphipod::B.energy() * 4 + Amphipod::A.energy() * 4
        );

        let mut state = State::<2>::goal();
        state.rooms[0][0] = Some(Amphipod::B);
        state.rooms[1][0] = None;
        state.hallway[0] = Some(Amphipod::A);
        assert_eq!(
            state.heuristics(),
            Amphipod::B.energy() * 4 + Amphipod::A.energy() * 3
        );
    }

    #[test]
    fn test_p1_sample() {
        let raw = include_str!(SAMPLE_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 12521);
    }

    #[test]
    fn test_p1() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p1(&input), 13558);
    }

    #[test]
    fn test_p2_sample() {
        let raw = include_str!(SAMPLE_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 44169);
    }

    #[test]
    fn test_p2() {
        let raw = include_str!(DATA_PATH!());
        let input = process(raw);
        assert_eq!(p2(&input), 56982);
    }
}
