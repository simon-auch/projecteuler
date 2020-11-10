#![feature(map_first_last)]
use projecteuler::helper;

fn main() {
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
    new.insert(start);
    seen.insert(start);

    //let moves = Cube::get_basic_moves(2);
    let moves = Cube::get_all_moves(2);

    while let Some(cube) = new.pop_first() {
        //dbg_cube(&cube);
        if seen.len() % 100000 == 0 {
            dbg!(seen.len(), new.len());
        }
        for movement in moves.iter() {
            //dbg_cube(&movement);
            let next = apply_move_2x2(&cube, movement);
            //dbg_cube(&next);
            if let None = seen.replace(next){
                new.insert(next);
            } else {
                //println!("hey");
            }
        }
        //return;
    }

    dbg!(seen.len());

    dbg!(solve(2));
}

fn solve(colors: usize) {
    //how many total cube arrangements can here be with n colors?
    /*
    2x2x2 cube has 24 faces
    n**24 would therefore be the total amount of possible initial colorings.
    It should be possible to shave of a few factors by pinning the first color etc
    */
    let mut seen = BitVec::with_capacity(colors.pow(24));
    let mut colorings = 0;
}

fn coloring_to_usize(cube: [u8;24], colors: usize) -> usize {
    cube.iter().fold(0, |acc, &c| acc * colors + c as usize)
}

struct BitVec{
    data: Vec<u8>,
}
impl BitVec {
    fn with_capacity(bits: usize) -> Self {
        Self{
            data: Vec::with_capacity((bits + 7)/8)
        }
    }
    
    fn is_set(&self, index: usize) -> bool {
        let high = index>>3;
        let low = index & 0b111;
        (self.data[high] >> low) & 0b1 == 1
    }

    fn set(&mut self, index: usize) {
        let high = index >> 3;
        let low = index & 0b111;
        self.data[high] |= 0b1 << low;
    }
}


fn apply_move_2x2(cube: &[u8;24], movement: &[usize]) -> [u8;24] {
    [
        cube[movement[0] ],
        cube[movement[1] ],
        cube[movement[2] ],
        cube[movement[3] ],
        cube[movement[4] ],
        cube[movement[5] ],
        cube[movement[6] ],
        cube[movement[7] ],
        cube[movement[8] ],
        cube[movement[9] ],
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

fn dbg_cube(cube: &[u8;24]){
    println!("
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
enum Side{
    F = 0,
    U = 1,
    R = 2,
    D = 3,
    L = 4,
    B = 5,
}
enum Rotation{
    Clockwise,
    Anticlockwise,
}

struct Move{
    side: Side,
    rotation: Rotation,
    layers: u8,
}

impl Side{
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
            _ => panic!("A Rubikscubs only has 6 sides..")
        }
    }
}

struct Cube{
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

impl Cube{
    fn get_identity_permutation(n: u8) -> Vec<usize>{
        //a cube width a side length of n has n*n*6 faces
        (0..((n*n*6) as usize)).collect()
    }
    fn get_basic_moves(n: u8) -> Vec<Vec<usize>>{
        let mut ret = Vec::with_capacity(((n-1)*3) as usize);
        let mut identity = std::collections::HashMap::new();
        for (i,coord) in Self::get_face_coordinates(n).enumerate(){
            //dbg!(i,coord);
            identity.insert(coord, i);
        }

        let n = n as isize;
        for depth in 1..=(n-1) {
            //the relevant cordinate must be greater than low
            let low = (n-depth)*2-(n+1);
            let mask_z = move |(_x,_y,z)| {z>low};
            let mask_x = move |(x,_y,_z)| {x>low};
            let mask_y = move |(_x,y,_z)| {y>low};

            //the following could totaly be made into a macro..
            let mut movement = Vec::new();
            movement.resize_with((n*n*6) as usize, || 0);
            Self::get_face_coordinates(n as u8).map(|orig|{
                if mask_x(orig) {
                    apply_rotation(orig, &ROTATION_X)
                } else {
                    orig
                }
            }).enumerate().for_each(|(i, now)|{
                movement[*identity.get(&now).unwrap()] = i;
            });
            ret.push(movement);

            let mut movement = Vec::new();
            movement.resize_with((n*n*6) as usize, || 0);
            Self::get_face_coordinates(n as u8).map(|orig|{
                if mask_y(orig) {
                    apply_rotation(orig, &ROTATION_Y)
                } else {
                    orig
                }
            }).enumerate().for_each(|(i, now)|{
                movement[*identity.get(&now).unwrap()] = i;
            });
            ret.push(movement);

            let mut movement = Vec::new();
            movement.resize_with((n*n*6) as usize, || 0);
            Self::get_face_coordinates(n as u8).map(|orig|{
                if mask_z(orig) {
                    apply_rotation(orig, &ROTATION_Z)
                } else {
                    orig
                }
            }).enumerate().for_each(|(i, now)|{
                movement[*identity.get(&now).unwrap()] = i;
            });
            ret.push(movement);
        }

        ret
    }
    fn get_all_moves(n: u8) -> Vec<Vec<usize>> {
        let mut ret: Vec<Vec<usize>> = Vec::with_capacity(((n-1)*6) as usize);
        ret.append(&mut Self::get_basic_moves(n));

        let mut identity = std::collections::HashMap::new();
        for (i,coord) in Self::get_face_coordinates(n).enumerate(){
            //dbg!(i,coord);
            identity.insert(coord, i);
        }

        let n = n as isize;
        for depth in 1..=(n-1) {
            //the relevant cordinate must be less or equal low
            let low = (n-depth)*2-(n+1);
            let mask_z = move |(_x,_y,z)| {z<=low};
            let mask_x = move |(x,_y,_z)| {x<=low};
            let mask_y = move |(_x,y,_z)| {y<=low};

            //the following could totaly be made into a macro..
            let mut movement = Vec::new();
            movement.resize_with((n*n*6) as usize, || 0);
            Self::get_face_coordinates(n as u8).map(|orig|{
                if mask_x(orig) {
                    apply_rotation(orig, &ROTATION_X)
                } else {
                    orig
                }
            }).enumerate().for_each(|(i, now)|{
                movement[*identity.get(&now).unwrap()] = i;
            });
            ret.push(movement);

            let mut movement = Vec::new();
            movement.resize_with((n*n*6) as usize, || 0);
            Self::get_face_coordinates(n as u8).map(|orig|{
                if mask_y(orig) {
                    apply_rotation(orig, &ROTATION_Y)
                } else {
                    orig
                }
            }).enumerate().for_each(|(i, now)|{
                movement[*identity.get(&now).unwrap()] = i;
            });
            ret.push(movement);

            let mut movement = Vec::new();
            movement.resize_with((n*n*6) as usize, || 0);
            Self::get_face_coordinates(n as u8).map(|orig|{
                if mask_z(orig) {
                    apply_rotation(orig, &ROTATION_Z)
                } else {
                    orig
                }
            }).enumerate().for_each(|(i, now)|{
                movement[*identity.get(&now).unwrap()] = i;
            });
            ret.push(movement);
        }
        ret
    }
    fn get_face_coordinates(n: u8) -> impl Iterator<Item=(isize,isize,isize)> {
        let n = n as isize;
        let down = (1..=n).map(move |i| (1..=n).map(move |j| (i,j)) ).flatten().map(
            move |(x,y)| (x,y,0)
        );
        let up = (1..=n).map(move |i| (1..=n).map(move |j| (i,j)) ).flatten().map(
            move |(x,y)| (x,y,n+1)
        );
        let left = (1..=n).map(move |i| (1..=n).map(move |j| (i,j)) ).flatten().map(
            move |(y,z)| (0,y,z)
        );
        let right = (1..=n).map(move |i| (1..=n).map(move |j| (i,j)) ).flatten().map(
            move |(y,z)| (n+1,y,z)
        );
        let front = (1..=n).map(move |i| (1..=n).map(move |j| (i,j)) ).flatten().map(
            move |(x,z)| (x,0,z)
        );
        let back = (1..=n).map(move |i| (1..=n).map(move |j| (i,j)) ).flatten().map(
            move |(x,z)| (x,n+1,z)
        );

        down.chain(up).chain(left).chain(right).chain(front).chain(back).map(move |(x,y,z)| (x*2-(n+1),y*2-(n+1),z*2-(n+1)))
    }
}

//roation matrix along the z axis (z coordinate stays the same)
static ROTATION_Z: [[isize;3];3] = [[
    0,-1,0
],[
    1,0,0
],[
    0,0,1
]];
static ROTATION_X: [[isize;3];3] = [[
    1,0,0
],[
    0,0,-1
],[
    0,1,0
]];
static ROTATION_Y: [[isize;3];3] = [[
    0,0,1
],[
    0,1,0
],[
    -1,0,0
]];

fn apply_rotation((x,y,z): (isize,isize,isize), rot: &[[isize;3];3]) -> (isize, isize, isize){
    (
        rot[0][0]*x + rot[0][1]*y + rot[0][2]*z,
        rot[1][0]*x + rot[1][1]*y + rot[1][2]*z,
        rot[2][0]*x + rot[2][1]*y + rot[2][2]*z,
    )
}