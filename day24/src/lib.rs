use glam::{DVec2, DVec3};
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1},
    combinator::map,
    multi::separated_list1,
    number::complete::double,
    sequence::{pair, separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Hailstone {
    pos: DVec3,
    vel: DVec3,
}

impl Hailstone {
    fn find_intersection_2d(&self, other: &Hailstone) -> Option<DVec2> {
        if self.pos == other.pos {
            return Some(DVec2::new(self.pos.x as f64, self.pos.y as f64));
        }
        let v1 = DVec2::new(self.vel.x as f64, self.vel.y as f64);
        let v2 = DVec2::new(other.vel.x as f64, other.vel.y as f64);
        if v1.angle_between(v2).abs() < 1e-10
            || (v1.angle_between(v2) - std::f64::consts::PI).abs() < 1e-5
        {
            return None;
        }
        let p1 = DVec2::new(self.pos.x as f64, self.pos.y as f64);
        let p2 = DVec2::new(other.pos.x as f64, other.pos.y as f64);
        let m1 = v1.y / v1.x;
        let m2 = v2.y / v2.x;

        // Calculate point of intersection
        let intersect_point = DVec2 {
            x: (m1 * p1.x - p1.y - m2 * p2.x + p2.y) / (m1 - m2),
            y: (m1 * p2.y + m1 * m2 * p1.x - m2 * p1.y - m1 * m2 * p2.x) / (m1 - m2),
        };

        // Figure out if the intersection is in the future path
        if v1.angle_between(intersect_point - p1).abs() < 1e-5
            && v2.angle_between(intersect_point - p2).abs() < 1e-5
        {
            Some(intersect_point)
        } else {
            None
        }
    }

    fn closest_distance(&self, other: &Hailstone) -> f64 {
        let normal = self.vel.cross(other.vel);
        if normal.length_squared() < 1e-5 {
            // Lines are parallel
            return ((other.pos - self.pos).cross(self.vel)).length() / other.vel.length();
        }
        normal.dot(self.pos - other.pos).abs() / normal.length()
    }
}

fn coords(text: &str) -> IResult<&str, DVec3> {
    map(
        tuple((
            double,
            pair(tag(","), space1),
            double,
            pair(tag(","), space1),
            double,
        )),
        |(x, _, y, _, z)| DVec3::new(x, y, z),
    )(text)
}

fn parse_hail(text: &str) -> IResult<&str, Vec<Hailstone>> {
    separated_list1(
        line_ending,
        map(
            separated_pair(coords, pair(tag(" @"), space1), coords),
            |(pos, vel)| Hailstone { pos, vel },
        ),
    )(text)
}

pub fn part1(text: String, min: f64, max: f64) -> usize {
    let (_, hail) = parse_hail(text.as_str()).unwrap();
    let mut num_intersections = 0;
    for (stone1, stone2) in hail.iter().tuple_combinations() {
        let intersection = stone1.find_intersection_2d(stone2);
        if let Some(loc) = intersection {
            if loc.x >= min && loc.x <= max && loc.y >= min && loc.y <= max {
                num_intersections += 1;
            }
        }
    }
    num_intersections
}

pub fn part2(text: String) -> i64 {
    let (_, hail) = parse_hail(text.as_str()).unwrap();
    todo!();
}
