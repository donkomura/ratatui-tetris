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
    pub shape: [[i32; 2]; 4], // [y, x]
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
    pub is_falling: bool,
    pub shape: Shape,       // shape of the tetromino
    pub position: [i32; 2], // position of the tetromino
}

impl Mino {
    pub fn new() -> Self {
        Mino {
            is_falling: false,
            shape: Shape::new(),
            position: [0, 0],
        }
    }
        }
    }
}

pub struct App {
    pub score: u64,
    pub should_quit: bool,
    pub mino: Mino,
    pub board: [[i32; 10]; 20], // 20x10 board
}

impl App {
    pub fn new() -> App {
        App {
            score: 0,
            should_quit: false,
            mino: Mino::new(),
            board: [[0; 10]; 20],
        }
    }
    // create new block at the top of the board
    pub fn spawn(&mut self) -> bool {
        let mino = Mino::new();
        let shape = mino.shape.shape;
        for i in 0..shape.len() {
            let [y, x] = shape[i];
            self.board[y as usize][x as usize] = 1;
        }
        self.mino = mino;
        return true;
    }
}
