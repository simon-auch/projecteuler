#![feature(map_first_last)]
use projecteuler::binomial;
use projecteuler::helper;

fn main() {
    assert_eq!(solve(2), 183);
    assert_eq!(solve(10), 12_395_526_079_546_335);
    dbg!(solve(2));
    dbg!(solve(10));
}

fn solve(colors: usize) -> usize {
    //What do I know
    /*
    2x2x2 cube has 24 faces
    n**24 would therefore be the total amount of possible initial colorings.
    It should be possible to shave of a few factors by pinning the first color etc

    holding one corner fixed a cube with side length 2 can reach a total of 3674160 configurations (each face colored differently)
    if not holding the corner fixed, there are a total of 88179840 configurations reachable (each face different)

    fact: 88179840/3674160 = 24 (6 sides * 4 rotations maybe?)

    all possible configurations a cube has have:
    colors ** 24

    all possible ways to rotate and place the cubelets:
    264539520 = 8! * 3**8 (8!: where each cubelet goes, 3**8 the rotation of each piece)

    fact: 8! * 3**8 / 88179840 = 3
    This means that given a cube coloring where each face is unique, there are exactly 3 equivalence groups

    given c colors, how many different ways to color a cubelet exist?
    *different also implies different after rotations (so 001 and 010 are the same)
    for 1 color : 000
    for 2 colors: 000, 111, 001, 011
    for 3 colors: 000, 111, 001, 011, 222, 002, 022, 112, 122, 012, 021
    T(1) = 1
    T(c) = T(c-1) + 1 + (c-1) * 2 + (c-1) * (c-2) * 2 / 2
         = T(c-1) + 1 + (c-1) * 2 + (c-1) * (c-2)
         = T(c-1) + 1 + (c-1) * (2 + (c-2))
         = T(c-1) + 1 + c * (c-1)

    given there are n essentially different colored cubelets, how many ways are there to choose 8?
    (This is basically "ziehen mit zurücklegen ohne reihenfolge")
    (n + 8 - 1) C (8)

    How many rotation invariant cubelets are there with c colors?
    c
    */
    /*
    println!("Cubelet colorings:");
    for c in 1..=10 {
        println!("c: {:02}, colorings: {}", c, cubelet_colorings(c));
    }
    println!("Cubelet colorings check:");
    for c in 1..=10 {
        println!("c: {:02}, colorings: {}", c, check_cubelet_colorings(c));
    }

    println!("cubelet selections:");
    for c in 1..=10 {
        println!("c: {:02}, cubelet selections: {}", c, binomial::binomial_coefficient(cubelet_colorings(c) + 7, 8));
    }

    println!("Cubelet selections without rotation invariant colorings:");
    for c in 2..=10 {
        println!("c: {:02}, cubelet selections: {}", c, binomial::binomial_coefficient(cubelet_colorings(c) - c as usize + 7, 8));
    }
    */

    let cubelet_colorings = cubelet_colorings(colors as u8);
    let cubelet_selections = binomial::binomial_coefficient(cubelet_colorings + 7, 8);
    let cubelet_selections_without_rotation_invariant_colorings =
        binomial::binomial_coefficient(cubelet_colorings - colors + 7, 8);
    //dbg!(cubelet_colorings, cubelet_selections, cubelet_selections_without_rotation_invariant_colorings);
    //every cubelet selection WITH a rotation invariant cublet coloring can reach all possible configurations
    //every cubelet selection WITHOUT a rotation invariant cublet coloring represents 3 different essential cube colorings (rotating one piece)
    cubelet_selections + cubelet_selections_without_rotation_invariant_colorings * 2
}

#[allow(unused)]
fn cube_configurations_brute_force() {
    #[rustfmt::skip]
    let start: [u8;24] = [
        0,0,
        0,0,
    1,1,2,2,3,3,4,4,
    1,1,2,2,3,3,4,4,
        5,5,
        5,5,
    ];
    #[rustfmt::skip]
    let start: [u8;24] = [
          00,01,
          02,03,
    04,05,06,07,08,09,10,11,
    12,13,14,15,16,17,18,19,
          20,21,
          22,23,
    ];
    let mut new = std::collections::BTreeSet::new();
    let mut seen = std::collections::HashSet::new();
    new.insert(compress_2x2(&start));
    seen.insert(compress_2x2(&start));

    let moves = Cube::get_basic_moves(2);
    //let moves = Cube::get_all_moves(2);

    while let Some(cube) = new.pop_first() {
        let cube = decompress_2x2(cube);
        //dbg_cube(&cube);
        if seen.len() % 100000 == 0 {
            dbg!(seen.len(), new.len());
        }
        for movement in moves.iter() {
            //dbg_cube(&movement);
            let next = apply_move_2x2(&cube, movement);
            let next = compress_2x2(&next);
            //dbg_cube(&next);
            if let None = seen.replace(next) {
                new.insert(next);
            } else {
                //println!("hey");
            }
        }
        //return;
    }

    dbg!(seen.len());
}

//returns the number of *different* cubelet colorings with c colors
fn cubelet_colorings(c: u8) -> usize {
    match c {
        1 => 1,
        c => cubelet_colorings(c - 1) + 1 + c as usize * (c - 1) as usize,
    }
}

fn check_cubelet_colorings(c: u8) -> usize {
    let mut acc = Vec::with_capacity((c as usize).pow(3));
    for c1 in 0..c {
        for c2 in 0..c {
            for c3 in 0..c {
                acc.push([c1, c2, c3]);
            }
        }
    }
    let mut i = 0;
    while i < acc.len() {
        let mut coloring = acc[i];
        for _ in 0..3 {
            for j in i + 1..acc.len() {
                while j < acc.len() && coloring == acc[j] {
                    acc.swap_remove(j);
                }
            }
            let temp = coloring[0];
            coloring[0] = coloring[1];
            coloring[1] = coloring[2];
            coloring[2] = temp;
        }
        i += 1;
    }
    //dbg!(&acc);
    acc.len()
}

fn compress_2x2(cube: &[u8; 24]) -> u128 {
    //there can be at the most 24 different colors -> 5 bit per face
    //24 faces -> 5*24 bit -> 120 bit
    cube.iter().rev().fold(0, |acc, &c| (acc << 5) | c as u128)
}

fn decompress_2x2(mut cube: u128) -> [u8; 24] {
    //just.. maybe.. a macro that repeats an arbitrary expression x times?
    [
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
        {
            let r = cube & 0b11111;
            cube >>= 5;
            r as u8
        },
    ]
}

fn apply_move_2x2(cube: &[u8; 24], movement: &[usize]) -> [u8; 24] {
    [
        cube[movement[0]],
        cube[movement[1]],
        cube[movement[2]],
        cube[movement[3]],
        cube[movement[4]],
        cube[movement[5]],
        cube[movement[6]],
        cube[movement[7]],
        cube[movement[8]],
        cube[movement[9]],
        cube[movement[10]],
        cube[movement[11]],
        cube[movement[12]],
        cube[movement[13]],
        cube[movement[14]],
        cube[movement[15]],
        cube[movement[16]],
        cube[movement[17]],
        cube[movement[18]],
        cube[movement[19]],
        cube[movement[20]],
        cube[movement[21]],
        cube[movement[22]],
        cube[movement[23]],
    ]
}

fn dbg_cube(cube: &[u8; 24]) {
    println!(
        "
      {:02},{:02},
      {:02},{:02},
{:02},{:02},{:02},{:02},{:02},{:02},{:02},{:02},
{:02},{:02},{:02},{:02},{:02},{:02},{:02},{:02},
      {:02},{:02},
      {:02},{:02},",
        cube[ID_2x2[00] as usize],
        cube[ID_2x2[01] as usize],
        cube[ID_2x2[02] as usize],
        cube[ID_2x2[03] as usize],
        cube[ID_2x2[04] as usize],
        cube[ID_2x2[05] as usize],
        cube[ID_2x2[06] as usize],
        cube[ID_2x2[07] as usize],
        cube[ID_2x2[08] as usize],
        cube[ID_2x2[09] as usize],
        cube[ID_2x2[10] as usize],
        cube[ID_2x2[11] as usize],
        cube[ID_2x2[12] as usize],
        cube[ID_2x2[13] as usize],
        cube[ID_2x2[14] as usize],
        cube[ID_2x2[15] as usize],
        cube[ID_2x2[16] as usize],
        cube[ID_2x2[17] as usize],
        cube[ID_2x2[18] as usize],
        cube[ID_2x2[19] as usize],
        cube[ID_2x2[20] as usize],
        cube[ID_2x2[21] as usize],
        cube[ID_2x2[22] as usize],
        cube[ID_2x2[23] as usize],
    )
}

//front, up, right, down, left, back
#[derive(Copy, Clone, Debug)]
enum Side {
    F = 0,
    U = 1,
    R = 2,
    D = 3,
    L = 4,
    B = 5,
}
enum Rotation {
    Clockwise,
    Anticlockwise,
}

struct Move {
    side: Side,
    rotation: Rotation,
    layers: u8,
}

impl Side {
    fn as_usize(self) -> usize {
        self as usize
    }
    fn from_usize(n: usize) -> Self {
        use Side::*;
        match n {
            0 => F,
            1 => U,
            2 => R,
            3 => D,
            4 => L,
            5 => B,
            _ => panic!("A Rubikscubs only has 6 sides.."),
        }
    }
}

struct Cube {
    data: Vec<u8>,
    side_length: u8,
}

#[rustfmt::skip]
static ID_2x2: &[u8;4*6] = &[
          00,01,
          02,03,
    04,05,06,07,08,09,10,11,
    12,13,14,15,16,17,18,19,
          20,21,
          22,23,
];
#[rustfmt::skip]
static MOVES_2x2: &[[u8;4*6];3] = &[
    [//Right, Clockwise
          00,07,
          02,15,
    04,05,06,21,16,08,03,11,
    12,13,14,23,17,09,01,19,
          20,18,
          22,10,
    ],
    [//Down, Clockwise
          00,01,
          02,03,
    04,05,06,07,08,09,10,11,
    18,19,12,13,14,15,16,17,
          22,20,
          23,21,
    ],
    [//Back, Clockwise
          09,17,
          02,03,
    01,05,06,07,08,23,18,10,
    00,13,14,15,16,22,19,11,
          20,21,
          04,12,
    ],
];

impl Cube {
    fn get_identity_permutation(n: u8) -> Vec<usize> {
        //a cube width a side length of n has n*n*6 faces
        (0..((n * n * 6) as usize)).collect()
    }
    fn get_basic_moves(n: u8) -> Vec<Vec<usize>> {
        let mut ret = Vec::with_capacity(((n - 1) * 3) as usize);
        let mut identity = std::collections::HashMap::new();
        for (i, coord) in Self::get_face_coordinates(n).enumerate() {
            //dbg!(i,coord);
            identity.insert(coord, i);
        }

        let n = n as isize;
        for depth in 1..=(n - 1) {
            //the relevant cordinate must be greater than low
            let low = (n - depth) * 2 - (n + 1);
            let mask_z = move |(_x, _y, z)| z > low;
            let mask_x = move |(x, _y, _z)| x > low;
            let mask_y = move |(_x, y, _z)| y > low;

            //the following could totaly be made into a macro..
            let mut movement = Vec::new();
            movement.resize_with((n * n * 6) as usize, || 0);
            Self::get_face_coordinates(n as u8)
                .map(|orig| {
                    if mask_x(orig) {
                        apply_rotation(orig, &ROTATION_X)
                    } else {
                        orig
                    }
                })
                .enumerate()
                .for_each(|(i, now)| {
                    movement[*identity.get(&now).unwrap()] = i;
                });
            ret.push(movement);

            let mut movement = Vec::new();
            movement.resize_with((n * n * 6) as usize, || 0);
            Self::get_face_coordinates(n as u8)
                .map(|orig| {
                    if mask_y(orig) {
                        apply_rotation(orig, &ROTATION_Y)
                    } else {
                        orig
                    }
                })
                .enumerate()
                .for_each(|(i, now)| {
                    movement[*identity.get(&now).unwrap()] = i;
                });
            ret.push(movement);

            let mut movement = Vec::new();
            movement.resize_with((n * n * 6) as usize, || 0);
            Self::get_face_coordinates(n as u8)
                .map(|orig| {
                    if mask_z(orig) {
                        apply_rotation(orig, &ROTATION_Z)
                    } else {
                        orig
                    }
                })
                .enumerate()
                .for_each(|(i, now)| {
                    movement[*identity.get(&now).unwrap()] = i;
                });
            ret.push(movement);
        }

        ret
    }
    fn get_all_moves(n: u8) -> Vec<Vec<usize>> {
        let mut ret: Vec<Vec<usize>> = Vec::with_capacity(((n - 1) * 6) as usize);
        ret.append(&mut Self::get_basic_moves(n));

        let mut identity = std::collections::HashMap::new();
        for (i, coord) in Self::get_face_coordinates(n).enumerate() {
            //dbg!(i,coord);
            identity.insert(coord, i);
        }

        let n = n as isize;
        for depth in 1..=(n - 1) {
            //the relevant cordinate must be less or equal low
            let low = (n - depth) * 2 - (n + 1);
            let mask_z = move |(_x, _y, z)| z <= low;
            let mask_x = move |(x, _y, _z)| x <= low;
            let mask_y = move |(_x, y, _z)| y <= low;

            //the following could totaly be made into a macro..
            let mut movement = Vec::new();
            movement.resize_with((n * n * 6) as usize, || 0);
            Self::get_face_coordinates(n as u8)
                .map(|orig| {
                    if mask_x(orig) {
                        apply_rotation(orig, &ROTATION_X)
                    } else {
                        orig
                    }
                })
                .enumerate()
                .for_each(|(i, now)| {
                    movement[*identity.get(&now).unwrap()] = i;
                });
            ret.push(movement);

            let mut movement = Vec::new();
            movement.resize_with((n * n * 6) as usize, || 0);
            Self::get_face_coordinates(n as u8)
                .map(|orig| {
                    if mask_y(orig) {
                        apply_rotation(orig, &ROTATION_Y)
                    } else {
                        orig
                    }
                })
                .enumerate()
                .for_each(|(i, now)| {
                    movement[*identity.get(&now).unwrap()] = i;
                });
            ret.push(movement);

            let mut movement = Vec::new();
            movement.resize_with((n * n * 6) as usize, || 0);
            Self::get_face_coordinates(n as u8)
                .map(|orig| {
                    if mask_z(orig) {
                        apply_rotation(orig, &ROTATION_Z)
                    } else {
                        orig
                    }
                })
                .enumerate()
                .for_each(|(i, now)| {
                    movement[*identity.get(&now).unwrap()] = i;
                });
            ret.push(movement);
        }
        ret
    }
    fn get_face_coordinates(n: u8) -> impl Iterator<Item = (isize, isize, isize)> {
        let n = n as isize;
        let down = (1..=n)
            .map(move |i| (1..=n).map(move |j| (i, j)))
            .flatten()
            .map(move |(x, y)| (x, y, 0));
        let up = (1..=n)
            .map(move |i| (1..=n).map(move |j| (i, j)))
            .flatten()
            .map(move |(x, y)| (x, y, n + 1));
        let left = (1..=n)
            .map(move |i| (1..=n).map(move |j| (i, j)))
            .flatten()
            .map(move |(y, z)| (0, y, z));
        let right = (1..=n)
            .map(move |i| (1..=n).map(move |j| (i, j)))
            .flatten()
            .map(move |(y, z)| (n + 1, y, z));
        let front = (1..=n)
            .map(move |i| (1..=n).map(move |j| (i, j)))
            .flatten()
            .map(move |(x, z)| (x, 0, z));
        let back = (1..=n)
            .map(move |i| (1..=n).map(move |j| (i, j)))
            .flatten()
            .map(move |(x, z)| (x, n + 1, z));

        down.chain(up)
            .chain(left)
            .chain(right)
            .chain(front)
            .chain(back)
            .map(move |(x, y, z)| (x * 2 - (n + 1), y * 2 - (n + 1), z * 2 - (n + 1)))
    }
}

//roation matrix along the z axis (z coordinate stays the same)
static ROTATION_Z: [[isize; 3]; 3] = [[0, -1, 0], [1, 0, 0], [0, 0, 1]];
static ROTATION_X: [[isize; 3]; 3] = [[1, 0, 0], [0, 0, -1], [0, 1, 0]];
static ROTATION_Y: [[isize; 3]; 3] = [[0, 0, 1], [0, 1, 0], [-1, 0, 0]];

fn apply_rotation(
    (x, y, z): (isize, isize, isize),
    rot: &[[isize; 3]; 3],
) -> (isize, isize, isize) {
    (
        rot[0][0] * x + rot[0][1] * y + rot[0][2] * z,
        rot[1][0] * x + rot[1][1] * y + rot[1][2] * z,
        rot[2][0] * x + rot[2][1] * y + rot[2][2] * z,
    )
}
