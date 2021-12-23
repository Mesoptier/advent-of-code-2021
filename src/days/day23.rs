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

fn abs_diff(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
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
    fn room_x(&self, room_index: usize) -> usize {
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

    fn transitions(&self) -> Vec<(State<R>, usize)> {
        let mut transitions = self.room_to_hallway_transitions();
        transitions.extend(self.hallway_to_room_transitions().into_iter());
        transitions
    }

    fn room_to_hallway_transitions(&self) -> Vec<(State<R>, usize)> {
        self.rooms.iter()
            .enumerate()
            .filter(|(room_index, _)| self.is_room_exitable(*room_index))
            .flat_map(|(room_index, room)| {
                // Find top-most amphipod
                // This always succeeds, because of the is_room_exitable check above
                let (room_depth, amphipod) = room.iter()
                    .enumerate()
                    .find_map(|(room_depth, space)| {
                        space.map(|amphipod| (room_depth, amphipod))
                    })
                    .unwrap();

                let current_x = self.room_x(room_index);

                // Step in either direction as long as there is empty space
                // TODO: Move this into a separate method
                let left_it = (0..current_x).rev()
                    .take_while(|x| self.hallway[*x].is_none());
                let right_it = ((current_x + 1)..self.hallway.len())
                    .take_while(|x| self.hallway[*x].is_none());
                left_it
                    .chain(right_it)
                    // Cannot move to a space directly above a room
                    .filter(|target_x| !self.is_above_room(*target_x))
                    .map(move |target_x| {
                        let steps = room_depth + 1 + abs_diff(current_x, target_x);
                        let energy = steps * amphipod.energy();

                        let mut state = *self;
                        std::mem::swap(
                            &mut state.rooms[room_index][room_depth],
                            &mut state.hallway[target_x],
                        );
                        (state, energy)
                    })
            })
            .collect()
    }

    fn hallway_to_room_transitions(&self) -> Vec<(State<R>, usize)> {
        self.hallway.iter()
            .enumerate()
            // Skip empty spaces
            .filter_map(|(current_x, space)| {
                space.map(|amphipod| (current_x, amphipod))
            })
            .filter_map(|(current_x, amphipod)| {
                let target_room_index = amphipod.target_room_index();

                if !self.is_room_enterable(target_room_index) {
                    // Target room still has other amphipods in it
                    return None;
                }

                let target_x = self.room_x(target_room_index);

                if !self.is_hallway_clear(current_x, target_x) {
                    // Cannot move through other amphipods
                    return None;
                }

                let target_room_depth = self.rooms[target_room_index].iter()
                    .rposition(|space| space.is_none())
                    .unwrap();

                let steps = target_room_depth + 1 + abs_diff(current_x, target_x);
                let energy = steps * amphipod.energy();

                let mut state = *self;
                std::mem::swap(
                    &mut state.rooms[target_room_index][target_room_depth],
                    &mut state.hallway[current_x],
                );

                Some((state, energy))
            })
            .collect()
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

                                let current_x = self.room_x(room_index);
                                let target_x = self.room_x(target_room_index);
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
                        let target_x = self.room_x(target_room_index);
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

        for (next_state, delta_energy) in state.transitions() {
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
