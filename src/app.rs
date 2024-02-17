use rand::Rng;

pub enum TETROMINOS {
    // all mino consists of 4 blocks
    STRAIGHT,
    SQUARE,
    T,
    L,
    S,
}

pub struct Shape {
    pub kind: TETROMINOS,
    pub shape: [[i32; 2]; 4],
}

impl Shape {
    fn straight() -> Self {
        Shape {
            kind: TETROMINOS::STRAIGHT,
            shape: [[0, 0], [0, 1], [0, 2], [0, 3]],
        }
    }
    fn square() -> Self {
        Shape {
            kind: TETROMINOS::SQUARE,
            shape: [[0, 0], [0, 1], [1, 0], [1, 1]],
        }
    }
    fn t() -> Self {
        Shape {
            kind: TETROMINOS::T,
            shape: [[0, 0], [0, 1], [0, 2], [1, 1]],
        }
    }
    fn l() -> Self {
        Shape {
            kind: TETROMINOS::L,
            shape: [[0, 0], [0, 1], [0, 2], [1, 2]],
        }
    }
    fn s() -> Self {
        Shape {
            kind: TETROMINOS::S,
            shape: [[0, 0], [0, 1], [1, 1], [1, 2]],
        }
    }
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen::<u8>() % 5 {
            0 => Shape::straight(),
            1 => Shape::square(),
            2 => Shape::t(),
            3 => Shape::l(),
            _ => Shape::s(),
        }
    }
}

pub struct Mino {
    pub shape: Shape, // shape of the tetromino
    pub cy: i32,      // conter y of the tetromino
    pub cx: i32,      // center x of the tetromino
}

impl Mino {
    pub fn new() -> Self {
        Mino {
            shape: Shape::new(),
            cy: 0,
            cx: 0,
        }
    }
}

pub struct App {
    pub score: u64,
    pub mino: Mino,
}

impl App {
    pub fn new() -> App {
        App {
            score: 0,
            mino: Mino::new(),
        }
    }
}
