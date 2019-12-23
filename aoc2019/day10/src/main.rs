use std::fs;
use std::fmt;
use std::cmp;
use std::collections::HashSet;


#[derive(Debug)]
struct Starfield {
    data: Vec<Vec<bool>>
}

impl fmt::Display for Starfield {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = self.data.iter()
            .map(|line| line.iter()
                 .map(|x| if *x { "#" } else { "." })
                 .collect::<Vec<&str>>()
                 .join("")
            )
            .collect::<Vec<String>>()
            .join("\n");

        f.write_str(result.as_str())
    }
}

impl Starfield {
    fn from_string(field: &String) -> Self {
        let data = field.lines()
            .map(|line| line.chars().map(|x| x == '#').collect())
            .collect();

        Starfield { data: data }
    }

    fn asteroids<'a>(&'a self) -> impl Iterator<Item=(usize, usize)> + 'a {
        self.data.iter()
            .enumerate()
            .flat_map(|(row_num, row)| row.iter()
                      .enumerate()
                      .filter(|(_, column)| **column)
                      .map(move |(column_num, _)| (column_num, row_num))
            )
    }

    fn visible(&self, asteroid: (usize, usize)) -> Vec<(usize, usize)> {
        let asteroids: HashSet<_> = self.asteroids().collect();

        let reference = (asteroid.0 as i32, asteroid.1 as i32);

        let mut visible = Vec::new();
        for ta in asteroids.clone() {
            if ta == asteroid {
                continue;
            }

            let target = (ta.0 as i32, ta.1 as i32);

            let blocker = asteroids.iter()
                .map(|blocker| (blocker.0 as i32, blocker.1 as i32))
                .filter(|blocker| *blocker != reference && *blocker != target)
                .filter(|blocker| is_blocked_by(reference, target, *blocker))
                .nth(0);

            if blocker.is_none() {
                visible.push(ta);
            }
        }

        visible
    }

    fn destroy(&mut self, asteroid: (usize, usize)) {
        self.data[asteroid.1][asteroid.0] = false;
    }
}


fn is_blocked_by(reference: (i32, i32), target: (i32, i32), blocker: (i32, i32)) -> bool {
    // in one line (X is same)
    if reference.0 == target.0 && reference.0 == blocker.0 {
        // blocker is between reference and target
        return between(blocker.1, reference.1, target.1);
    }
    // in one line (Y is same)
    if reference.1 == target.1 && reference.1 == blocker.1 {
        // blocker is between reference and target
        return between(blocker.0, reference.0, target.0);
    }

    let rx: f32 = reference.0 as f32;
    let ry: f32 = reference.1 as f32;
    let tx: f32 = target.0 as f32;
    let ty: f32 = target.1 as f32;
    let bx: f32 = blocker.0 as f32;
    let by: f32 = blocker.1 as f32;

    let m = (ty - ry) / (tx - rx);
    let n = (ry*tx - ty * rx) / (tx - rx);

    // println!("m: {} | n: {} | angle: {}", m, n, angle(reference, target));

    let on_line = ((m*bx + n) - by).abs() < 0.01;

    on_line
        && between(blocker.0, reference.0, target.0)
        && between(blocker.1, reference.1, target.1)
}



fn angle(reference: (i32, i32), target: (i32, i32)) -> f32 {
    let rx: f32 = reference.0 as f32;
    let ry: f32 = reference.1 as f32;
    let tx: f32 = target.0 as f32;
    let ty: f32 = target.1 as f32;

    // move angles so 0 is "top"
    let r = f32::atan2(ty - ry, tx - rx) + (std::f32::consts::PI / 2.);

    if r < 0. {
        // fix the ordering so top left is actually last
        r + (std::f32::consts::PI * 2.)
    } else {
        r
    }
}


fn between(num: i32, a: i32, b: i32) -> bool {
    num >= cmp::min(a, b) && num <= cmp::max(a, b)
}


fn main() {
    let input = fs::read_to_string("../input.txt").unwrap();

    let mut starfield = Starfield::from_string(&input);

    let ideal_astroid = starfield.asteroids()
        .max_by_key(|a| starfield.visible(*a).len())
        .unwrap();

    println!("{}", starfield);
    println!("ideal asteroid: {:?}", ideal_astroid);
    println!("visible asteroids: {:?}", starfield.visible(ideal_astroid).len());

    let mut i = 0;
    let mut result = None;
    loop {
        let mut f = starfield.visible(ideal_astroid);
        f.sort_by(|a, b| {
            let ideal = (ideal_astroid.0 as i32, ideal_astroid.1 as i32);
            let angle_a = angle(ideal, (a.0 as i32, a.1 as i32));
            let angle_b = angle(ideal, (b.0 as i32, b.1 as i32));

            angle_a.partial_cmp(&angle_b).unwrap()
        });

        for a in f {
            starfield.destroy(a);
            i = i + 1;
            if i == 200 {
                result = Some(a);
            }
        }

        if starfield.asteroids().nth(1).is_none() {
            break;
        }
    }

    println!("200th destroyed asteroid: {:?}", result);
}


#[test]
fn test() {
    assert_eq!(is_blocked_by((3, 4), (1, 2), (2, 2)), false);
    assert_eq!(is_blocked_by((3, 4), (0, 2), (1, 2)), false);

    assert_eq!(is_blocked_by((4, 0), (4, 4), (4, 2)), true);
    assert_eq!(is_blocked_by((0, 4), (4, 4), (2, 4)), true);
    assert_eq!(is_blocked_by((4, 0), (4, 2), (4, 4)), false);
    assert_eq!(is_blocked_by((0, 4), (2, 4), (4, 4)), false);

    assert_eq!(is_blocked_by((0, 0), (2, 2), (4, 4)), false);
    assert_eq!(is_blocked_by((0, 0), (4, 4), (2, 2)), true);
    assert_eq!(is_blocked_by((3, 2), (9, 6), (6, 4)), true);
    assert_eq!(is_blocked_by((1, 1), (9, 9), (6, 6)), true);
}

