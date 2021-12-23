use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::{Display, Formatter};

use hashbrown::HashMap;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Amphipod {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl Amphipod {
    fn energy(&self) -> usize {
        10usize.pow(*self as u32)
    }

    fn target_room_index(&self) -> usize {
        *self as usize
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct State<const R: usize> {
    // TODO: Shrink this array to 7 elements, since spaces above rooms are invalid
    hallway: [Option<Amphipod>; 11],
    rooms: [[Option<Amphipod>; R]; 4],
}

impl<const R: usize> State<R> {
    fn is_complete(&self) -> bool {
        self.rooms.iter().enumerate().all(|(room_index, room)| {
            room.iter().all(|space| match space {
                None => false,
                Some(amphipod) => amphipod.target_room_index() == room_index,
            })
        })
    }

    /// Whether the room with the given index can be entered (by a matching amphipod).
    fn is_room_enterable(&self, room_index: usize) -> bool {
        self.rooms[room_index].iter().all(|space| {
            match space {
                None => true,
                Some(amphipod) => amphipod.target_room_index() == room_index,
            }
        })
    }

    fn is_room_exitable(&self, room_index: usize) -> bool {
        !self.is_room_enterable(room_index)
    }

    /// Maps from room index to hallway position of the space above the room.
    fn room_to_hallway(&self, room_index: usize) -> usize {
        2 + (room_index) * 2
    }

    fn is_above_room(&self, x: usize) -> bool {
        (x - 2) % 2 == 0
            && (x - 2) / 2 < self.rooms.len()
    }

    /// Check if an amphipod at start_x can freely move to target_x.
    fn is_hallway_clear(&self, start_x: usize, target_x: usize) -> bool {
        let slice = match start_x.cmp(&target_x) {
            Ordering::Equal => {
                return true;
            }
            Ordering::Less => &self.hallway[(start_x + 1)..=target_x],
            Ordering::Greater => &self.hallway[target_x..start_x],
        };

        slice.iter().all(|space| space.is_none())
    }

    fn adjacent_states(&self) -> Vec<(State<R>, usize)> {
        let mut states = vec![];

        // (room -> room), (room -> hallway)
        for (room_index, room) in self.rooms.iter().enumerate() {
            if !self.is_room_exitable(room_index) {
                continue;
            }

            for (room_depth, space) in room.iter().enumerate() {
                match space {
                    None => {}
                    Some(amphipod) => {
                        let current_x = self.room_to_hallway(room_index);
                        let steps = room_depth + 1;

                        // Move to target room
                        let target_room_index = amphipod.target_room_index();
                        if self.is_room_enterable(target_room_index) {
                            assert_ne!(target_room_index, room_index);

                            let target_x = self.room_to_hallway(target_room_index);
                            if self.is_hallway_clear(current_x, target_x) {
                                let target_room_depth = self.rooms[target_room_index].iter()
                                    .rposition(|space| space.is_none())
                                    .unwrap();

                                let hallway_steps = if current_x < target_x {
                                    target_x - current_x
                                } else {
                                    current_x - target_x
                                };

                                let steps = steps + (target_room_depth + 1) + hallway_steps;

                                let mut state = *self;
                                state.rooms[target_room_index][target_room_depth] = Some(*amphipod);
                                state.rooms[room_index][room_depth] = None;
                                states.push((state, steps * amphipod.energy()));
                            }
                        }

                        // Move to hallway
                        for target_x in 0..self.hallway.len() {
                            if self.is_above_room(target_x) {
                                continue;
                            }

                            if self.is_hallway_clear(current_x, target_x) {
                                let hallway_steps = if current_x < target_x {
                                    target_x - current_x
                                } else {
                                    current_x - target_x
                                };

                                let steps = steps + hallway_steps;

                                let mut state = *self;
                                state.hallway[target_x] = Some(*amphipod);
                                state.rooms[room_index][room_depth] = None;
                                states.push((state, steps * amphipod.energy()));
                            }
                        }

                        // Only the top-most amphipod can move out of a room
                        break;
                    }
                }
            }
        }

        // (hallway -> room)
        for (current_x, space) in self.hallway.iter().enumerate() {
            match space {
                None => {}
                Some(amphipod) => {
                    let target_room_index = amphipod.target_room_index();
                    if self.is_room_enterable(target_room_index) {
                        let target_x = self.room_to_hallway(target_room_index);
                        if self.is_hallway_clear(current_x, target_x) {
                            let target_room_depth = self.rooms[target_room_index].iter()
                                .rposition(|space| space.is_none())
                                .unwrap();

                            let hallway_steps = if current_x < target_x {
                                target_x - current_x
                            } else {
                                current_x - target_x
                            };

                            let steps = (target_room_depth + 1) + hallway_steps;

                            let mut state = *self;
                            state.rooms[target_room_index][target_room_depth] = Some(*amphipod);
                            state.hallway[current_x] = None;
                            states.push((state, steps * amphipod.energy()));
                        }
                    }
                }
            }
        }

        states
    }

    fn h_score(&self) -> usize {
        let room_room = self.rooms.iter()
            .enumerate()
            .flat_map(|(room_index, room)| {
                room.iter()
                    .enumerate()
                    .map(move |(room_depth, space)| {
                        match space {
                            None => 0,
                            Some(amphipod) => {
                                let target_room_index = amphipod.target_room_index();

                                if room_index == target_room_index {
                                    return 0;
                                }

                                let current_x = self.room_to_hallway(room_index);
                                let target_x = self.room_to_hallway(target_room_index);
                                let hallway_steps = if current_x < target_x {
                                    target_x - current_x
                                } else {
                                    current_x - target_x
                                };

                                let steps = hallway_steps + room_depth + 2;
                                steps * amphipod.energy()
                            }
                        }
                    })
            })
            .sum::<usize>();

        let hallway_room = self.hallway.iter()
            .enumerate()
            .map(|(current_x, space)| {
                match space {
                    None => 0,
                    Some(amphipod) => {
                        let target_room_index = amphipod.target_room_index();
                        let target_x = self.room_to_hallway(target_room_index);
                        let hallway_steps = if current_x < target_x {
                            target_x - current_x
                        } else {
                            current_x - target_x
                        };

                        let steps = hallway_steps + 1;
                        steps * amphipod.energy()
                    }
                }
            })
            .sum::<usize>();

        room_room + hallway_room
    }
}

impl<const R: usize> Display for State<R> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let space_to_str = |space: Option<Amphipod>| -> &str {
            match space {
                None => ".",
                Some(Amphipod::A) => "A",
                Some(Amphipod::B) => "B",
                Some(Amphipod::C) => "C",
                Some(Amphipod::D) => "D",
            }
        };

        writeln!(f, "{}", "#".repeat(self.hallway.len() + 2))?;
        writeln!(f, "#{}#", self.hallway.map(space_to_str).join(""))?;
        writeln!(f, "###{}###", self.rooms.map(|r| space_to_str(r[0])).join("#"))?;
        for room_depth in 1..R {
            writeln!(f, "  #{}#  ", self.rooms.map(|r| space_to_str(r[room_depth])).join("#"))?;
        }
        write!(f, "  {}  ", "#".repeat(self.rooms.len() * 2 + 1))?;

        Ok(())
    }
}

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<Amphipod> {
    let amphipods = input.chars()
        .filter_map(|c| match c {
            'A' => Some(Amphipod::A),
            'B' => Some(Amphipod::B),
            'C' => Some(Amphipod::C),
            'D' => Some(Amphipod::D),
            _ => None,
        })
        .collect::<Vec<_>>();

    assert_eq!(amphipods.len(), 8);

    amphipods
}

#[derive(PartialEq, Eq)]
struct Entry<const R: usize> {
    state: State<R>,
    f_score: usize,
}

impl<const R: usize> PartialOrd<Self> for Entry<R> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<const R: usize> Ord for Entry<R> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f_score.cmp(&other.f_score).reverse()
    }
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &Vec<Amphipod>) -> usize {
    let initial_state = State {
        hallway: [None; 11],
        rooms: [
            [Some(input[0]), Some(input[4])],
            [Some(input[1]), Some(input[5])],
            [Some(input[2]), Some(input[6])],
            [Some(input[3]), Some(input[7])],
        ],
    };

    solve_both_parts(initial_state)
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &Vec<Amphipod>) -> usize {
    let initial_state = State {
        hallway: [None; 11],
        rooms: [
            [input[0], Amphipod::D, Amphipod::D, input[4]].map(|a| Some(a)),
            [input[1], Amphipod::C, Amphipod::B, input[5]].map(|a| Some(a)),
            [input[2], Amphipod::B, Amphipod::A, input[6]].map(|a| Some(a)),
            [input[3], Amphipod::A, Amphipod::C, input[7]].map(|a| Some(a)),
        ],
    };

    solve_both_parts(initial_state)
}

fn solve_both_parts<const R: usize>(initial_state: State<R>) -> usize {
    let mut q = BinaryHeap::new();
    q.push(Entry {
        state: initial_state,
        f_score: 0,
    });

    let mut g_score: HashMap<State<R>, usize> = HashMap::new();
    g_score.insert(initial_state, 0);

    while let Some(Entry { state, f_score }) = q.pop() {
        if state.is_complete() {
            return f_score;
        }

        for (next_state, delta_energy) in state.adjacent_states() {
            let tentative_g_score = g_score[&state] + delta_energy;
            if tentative_g_score < *g_score.get(&next_state).unwrap_or(&usize::MAX) {
                g_score.insert(next_state, tentative_g_score);
                q.push(Entry {
                    state: next_state,
                    f_score: tentative_g_score + next_state.h_score(),
                });
            }
        }
    }

    unreachable!();
}
