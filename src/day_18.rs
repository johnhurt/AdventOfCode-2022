use std::{collections::HashSet, fmt::Display};

use iter_tools::Itertools;
use nom::{
    character::complete::{char, i32},
    sequence::{terminated, tuple},
    IResult,
};

fn parse_voxel(line: String) -> (i32, i32, i32) {
    let result: IResult<&str, (i32, i32, i32)> =
        tuple((terminated(i32, char(',')), terminated(i32, char(',')), i32))(
            &line,
        );

    result.expect("🫥").1
}

fn all_connected(
    min: (i32, i32, i32),
    max: (i32, i32, i32),
    voxels: &HashSet<(i32, i32, i32)>,
    exterior: &HashSet<(i32, i32, i32)>,
    start: (i32, i32, i32),
) -> (Vec<(i32, i32, i32)>, bool) {
    let mut visited = HashSet::new();
    let mut stack = vec![start];
    let mut ext = false;

    while !stack.is_empty() {
        let p = stack.pop().expect("💀");

        visited.insert(p);
        if exterior.contains(&p) {
            ext = true
        }

        stack.extend(
            [
                (p.0 + 1, p.1, p.2),
                (p.0 - 1, p.1, p.2),
                (p.0, p.1 + 1, p.2),
                (p.0, p.1 - 1, p.2),
                (p.0, p.1, p.2 + 1),
                (p.0, p.1, p.2 - 1),
            ]
            .iter()
            .filter(|n| voxels.contains(n))
            .filter(|n| !visited.contains(*n)),
        );
    }

    (visited.into_iter().collect_vec(), ext)
}

fn find_interior_space_surface(
    min: (i32, i32, i32),
    max: (i32, i32, i32),
    voxels: HashSet<(i32, i32, i32)>,
) -> i32 {
    let mut non_voxels = HashSet::new();
    let mut exterior = HashSet::new();

    for x in min.0..=max.0 {
        for y in min.1..=max.1 {
            for z in min.2..=max.2 {
                let p = (x, y, z);

                if !voxels.contains(&p) {
                    let ext = x == min.0
                        || y == min.1
                        || z == min.2
                        || x == max.0
                        || y == max.1
                        || z == max.2;
                    non_voxels.insert(p);
                    if ext {
                        exterior.insert(p);
                    }
                }
            }
        }
    }

    let mut total_surface_area = 0;

    while !non_voxels.is_empty() {
        let curr = *non_voxels.iter().next().expect("🤠");

        let (all_connected, ext) =
            all_connected(min, max, &non_voxels, &exterior, curr);

        all_connected.iter().for_each(|v| {
            non_voxels.remove(v);
        });

        if !ext {
            let mut anti_voxels = HashSet::new();
            let mut surface_area = 0_i32;

            for p in all_connected.into_iter() {
                anti_voxels.insert(p);
                let neighbors = [
                    (p.0 + 1, p.1, p.2),
                    (p.0 - 1, p.1, p.2),
                    (p.0, p.1 + 1, p.2),
                    (p.0, p.1 - 1, p.2),
                    (p.0, p.1, p.2 + 1),
                    (p.0, p.1, p.2 - 1),
                ]
                .into_iter()
                .filter(|v| anti_voxels.contains(v))
                .count();

                surface_area += 6 - 2 * neighbors as i32;
            }

            total_surface_area += surface_area;
        }
    }

    total_surface_area
}

/**** Problem 1 ******/

pub fn problem_1<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut voxels = HashSet::new();
    let mut surface_area = 0_i32;

    for p in input_lines.map(parse_voxel) {
        voxels.insert(p);
        let neighbors = [
            (p.0 + 1, p.1, p.2),
            (p.0 - 1, p.1, p.2),
            (p.0, p.1 + 1, p.2),
            (p.0, p.1 - 1, p.2),
            (p.0, p.1, p.2 + 1),
            (p.0, p.1, p.2 - 1),
        ]
        .into_iter()
        .filter(|v| voxels.contains(v))
        .count();

        surface_area += 6 - 2 * neighbors as i32;
    }

    surface_area
}

/**** Problem 2 ******/

pub fn problem_2<I>(input_lines: I) -> impl Display
where
    I: Iterator<Item = String>,
{
    let mut voxels = HashSet::new();
    let mut surface_area = 0_i32;
    let mut max = (i32::MIN, i32::MIN, i32::MIN);
    let mut min = (i32::MAX, i32::MAX, i32::MAX);

    for p in input_lines.map(parse_voxel) {
        min = (min.0.min(p.0), min.1.min(p.1), min.2.min(p.2));
        max = (max.0.max(p.0), max.1.max(p.1), max.2.max(p.2));

        voxels.insert(p);
        let neighbors = [
            (p.0 + 1, p.1, p.2),
            (p.0 - 1, p.1, p.2),
            (p.0, p.1 + 1, p.2),
            (p.0, p.1 - 1, p.2),
            (p.0, p.1, p.2 + 1),
            (p.0, p.1, p.2 - 1),
        ]
        .into_iter()
        .filter(|v| voxels.contains(v))
        .count();

        surface_area += 6 - 2 * neighbors as i32;
    }

    surface_area - find_interior_space_surface(min, max, voxels)
}
