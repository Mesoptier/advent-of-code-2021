use std::slice::Iter;
use std::str::FromStr;

use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, newline};
use nom::combinator::{map, map_res, opt, recognize};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, tuple};

#[derive(Copy, Clone, Debug)]
pub struct Cuboid {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    z1: i32,
    z2: i32,
}

impl Cuboid {
    fn lower(&self, axis: Axis) -> i32 {
        match axis {
            Axis::X => self.x1,
            Axis::Y => self.y1,
            Axis::Z => self.z1,
        }
    }
    fn lower_mut(&mut self, axis: Axis) -> &mut i32 {
        match axis {
            Axis::X => &mut self.x1,
            Axis::Y => &mut self.y1,
            Axis::Z => &mut self.z1,
        }
    }

    fn upper(&self, axis: Axis) -> i32 {
        match axis {
            Axis::X => self.x2,
            Axis::Y => self.y2,
            Axis::Z => self.z2,
        }
    }
    fn upper_mut(&mut self, axis: Axis) -> &mut i32 {
        match axis {
            Axis::X => &mut self.x2,
            Axis::Y => &mut self.y2,
            Axis::Z => &mut self.z2,
        }
    }

    fn contains(&self, other: &Cuboid) -> bool {
        Axis::iter().all(|&axis| {
            self.lower(axis) <= other.lower(axis)
                && other.upper(axis) <= self.upper(axis)
        })
    }

    fn intersection(&self, other: &Cuboid) -> Option<Cuboid> {
        let x1 = self.x1.max(other.x1);
        let x2 = self.x2.min(other.x2);
        let y1 = self.y1.max(other.y1);
        let y2 = self.y2.min(other.y2);
        let z1 = self.z1.max(other.z1);
        let z2 = self.z2.min(other.z2);

        if x1 <= x2 && y1 <= y2 && z1 <= z2 {
            Some(Cuboid {
                x1,
                x2,
                y1,
                y2,
                z1,
                z2,
            })
        } else {
            None
        }
    }

    fn size(&self) -> usize {
        Axis::iter().map(|&axis| {
            (1 + self.upper(axis) - self.lower(axis)) as usize
        }).product::<usize>()
    }

    fn split(&self, split_axis: SplitAxis) -> (Option<Cuboid>, Option<Cuboid>) {
        let (axis, c) = split_axis;

        if self.upper(axis) < c {
            (Some(*self), None)
        } else if c <= self.lower(axis) {
            (None, Some(*self))
        } else {
            let mut left_cuboid = *self;
            *left_cuboid.upper_mut(axis) = c - 1;
            let mut right_cuboid = *self;
            *right_cuboid.lower_mut(axis) = c;
            (Some(left_cuboid), Some(right_cuboid))
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn iter() -> Iter<'static, Axis> {
        static AXES: [Axis; 3] = [Axis::X, Axis::Y, Axis::Z];
        AXES.iter()
    }
}

type SplitAxis = (Axis, i32);

#[derive(Clone, Debug)]
enum CuboidNode {
    Nil,
    Leaf(Cuboid),
    Split {
        split_axis: SplitAxis,
        left: Box<CuboidNode>,
        right: Box<CuboidNode>,
    },
}

impl CuboidNode {
    fn add(&mut self, cuboid: Cuboid) {
        match self {
            CuboidNode::Nil => {
                *self = CuboidNode::Leaf(cuboid);
            }
            CuboidNode::Split { split_axis, left, right } => {
                let (left_cuboid, right_cuboid) = cuboid.split(*split_axis);
                if let Some(left_cuboid) = left_cuboid {
                    left.add(left_cuboid);
                }
                if let Some(right_cuboid) = right_cuboid {
                    right.add(right_cuboid);
                }
            }
            CuboidNode::Leaf(leaf_cuboid) => {
                // Handle cases where cuboids completely overlap
                if leaf_cuboid.contains(&cuboid) {
                    return;
                }
                if cuboid.contains(leaf_cuboid) {
                    *leaf_cuboid = cuboid;
                    return;
                }

                // Handle cases where cuboids are completely disjoint
                for &axis in Axis::iter() {
                    if cuboid.upper(axis) < leaf_cuboid.lower(axis) {
                        let split_axis: SplitAxis = (axis, leaf_cuboid.lower(axis));
                        *self = CuboidNode::Split {
                            split_axis,
                            left: CuboidNode::Leaf(cuboid).into(),
                            right: CuboidNode::Leaf(*leaf_cuboid).into(),
                        };
                        return;
                    }
                    if leaf_cuboid.upper(axis) < cuboid.lower(axis) {
                        let split_axis: SplitAxis = (axis, cuboid.lower(axis));
                        *self = CuboidNode::Split {
                            split_axis,
                            left: CuboidNode::Leaf(*leaf_cuboid).into(),
                            right: CuboidNode::Leaf(cuboid).into(),
                        };
                        return;
                    }
                }

                // Handle cases where cuboids partially overlap (complete overlap is handled above)
                for &axis in Axis::iter() {
                    if let Some((split_axis, left_node, right_node)) = {
                        if cuboid.lower(axis) < leaf_cuboid.lower(axis) {
                            let split_axis: SplitAxis = (axis, leaf_cuboid.lower(axis));
                            let (left_cuboid, right_cuboid) = cuboid.split(split_axis);

                            let left_node = CuboidNode::Leaf(left_cuboid.unwrap());
                            let mut right_node = CuboidNode::Leaf(*leaf_cuboid);
                            right_node.add(right_cuboid.unwrap());

                            Some((split_axis, left_node, right_node))
                        } else if leaf_cuboid.upper(axis) < cuboid.upper(axis) {
                            let split_axis: SplitAxis = (axis, leaf_cuboid.upper(axis) + 1);
                            let (left_cuboid, right_cuboid) = cuboid.split(split_axis);

                            let mut left_node = CuboidNode::Leaf(*leaf_cuboid);
                            left_node.add(left_cuboid.unwrap());
                            let right_node = CuboidNode::Leaf(right_cuboid.unwrap());

                            Some((split_axis, left_node, right_node))
                        } else {
                            None
                        }
                    } {
                        *self = CuboidNode::Split {
                            split_axis,
                            left: left_node.into(),
                            right: right_node.into(),
                        };
                        return;
                    }
                }

                unreachable!();
            }
        }
    }

    fn sub(&mut self, cuboid: Cuboid) {
        match self {
            CuboidNode::Nil => {
                return;
            }
            CuboidNode::Split { split_axis, left, right } => {
                // TODO: Do we really need to split the cuboid here? Since it's a subtracting cuboid,
                //  the invariant won't be broken if this cuboid stays at its original size.
                let (left_cuboid, right_cuboid) = cuboid.split(*split_axis);
                if let Some(left_cuboid) = left_cuboid {
                    left.sub(left_cuboid);
                }
                if let Some(right_cuboid) = right_cuboid {
                    right.sub(right_cuboid);
                }

                // TODO: Simplify if left/right is Nil after calling .sub()
            }
            CuboidNode::Leaf(leaf_cuboid) => {
                if cuboid.contains(leaf_cuboid) {
                    *self = CuboidNode::Nil;
                    return;
                }

                // Handle cases where cuboids are completely disjoint
                for &axis in Axis::iter() {
                    if cuboid.upper(axis) < leaf_cuboid.lower(axis) {
                        return;
                    }
                    if leaf_cuboid.upper(axis) < cuboid.lower(axis) {
                        return;
                    }
                }

                // Handle cases where cuboids overlap
                for &axis in Axis::iter() {
                    if let Some((split_axis, left_node, right_node)) = {
                        if cuboid.upper(axis) < leaf_cuboid.upper(axis) {
                            let split_axis: SplitAxis = (axis, cuboid.upper(axis) + 1);
                            let (left_cuboid, right_cuboid) = leaf_cuboid.split(split_axis);

                            let mut left_node = CuboidNode::Leaf(left_cuboid.unwrap());
                            left_node.sub(cuboid);
                            let right_node = CuboidNode::Leaf(right_cuboid.unwrap());

                            Some((split_axis, left_node, right_node))
                        } else if leaf_cuboid.lower(axis) < cuboid.lower(axis) {
                            let split_axis: SplitAxis = (axis, cuboid.lower(axis));
                            let (left_cuboid, right_cuboid) = leaf_cuboid.split(split_axis);

                            let left_node = CuboidNode::Leaf(left_cuboid.unwrap());
                            let mut right_node = CuboidNode::Leaf(right_cuboid.unwrap());
                            right_node.sub(cuboid);

                            Some((split_axis, left_node, right_node))
                        } else {
                            None
                        }
                    } {
                        *self = CuboidNode::Split {
                            split_axis,
                            left: left_node.into(),
                            right: right_node.into(),
                        };
                        return;
                    }
                }

                unreachable!();
            }
        }
    }

    fn size(&self) -> usize {
        match self {
            CuboidNode::Nil => 0,
            CuboidNode::Leaf(cuboid) => cuboid.size(),
            CuboidNode::Split { left, right, .. } => {
                left.size() + right.size()
            }
        }
    }
}

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<(bool, Cuboid)> {
    fn parse_step(input: &str) -> IResult<&str, (bool, Cuboid)> {
        tuple((
            alt((
                map(tag("on"), |_| true),
                map(tag("off"), |_| false)
            )),
            map(tuple((
                preceded(tag(" x="), parse_range),
                preceded(tag(",y="), parse_range),
                preceded(tag(",z="), parse_range),
            )), |((x1, x2), (y1, y2), (z1, z2))| Cuboid {
                x1,
                x2,
                y1,
                y2,
                z1,
                z2,
            })
        ))(input)
    }

    fn parse_range(input: &str) -> IResult<&str, (i32, i32)> {
        separated_pair(
            parse_signed_int,
            tag(".."),
            parse_signed_int,
        )(input)
    }

    fn parse_signed_int<T: FromStr>(input: &str) -> IResult<&str, T> {
        map_res(
            recognize(tuple((opt(tag("-")), digit1))),
            FromStr::from_str,
        )(input)
    }

    separated_list1(
        newline,
        parse_step,
    )(input).unwrap().1
}

fn solve_both_parts(input: &Vec<(bool, Cuboid)>, part1: bool) -> usize {
    let mut tree = CuboidNode::Nil;

    let initialization_area = Cuboid {
        x1: -50,
        x2: 50,
        y1: -50,
        y2: 50,
        z1: -50,
        z2: 50,
    };

    for (state, mut cuboid) in input {
        // Part 1: intersect cuboid with intersection area, skip if intersection is empty
        if part1 {
            if let Some(intersection) = cuboid.intersection(&initialization_area) {
                cuboid = intersection;
            } else {
                continue;
            }
        }

        if *state {
            tree.add(cuboid);
        } else {
            tree.sub(cuboid);
        }
    }

    tree.size()
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &Vec<(bool, Cuboid)>) -> usize {
    solve_both_parts(input, true)
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &Vec<(bool, Cuboid)>) -> usize {
    solve_both_parts(input, false)
}
