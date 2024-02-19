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
    pub block: Shape, // shape of the tetromino
}

impl Mino {
    pub fn new() -> Self {
        Mino {
            is_falling: false,
            block: Shape::new(),
        }
    }
}

pub struct Point {
    pub y: i32,
    pub x: i32,
}

pub struct App {
    pub score: u64,
    pub should_quit: bool,
    pub mino: Mino,
    pub position: Point,        // left corner of the tetromino
    pub board: [[i32; 10]; 20], // 20x10 board
}

impl App {
    pub fn new() -> App {
        App {
            score: 0,
            should_quit: false,
            mino: Mino::new(),
            board: [[0; 10]; 20],
            position: Point { y: 0, x: 0 },
        }
    }
    pub fn reset_position(&mut self) {
        self.position = Point { y: 0, x: 0 };
    }
    fn is_out_of_range(&self, py: i32, px: i32) -> bool {
        if py < 0 || py >= 20 || px < 0 || px >= 10 {
            return true;
        }
        return false;
    }
    fn is_conflict(&self, mino: &Mino) -> bool {
        let shape = &mino.block.shape;
        let position = &self.position;
        for i in 0..shape.len() {
            let [y, x] = shape[i];
            let cy = y + position.y;
            let cx = x + position.x;
            if self.is_out_of_range(cy, cx) {
                return true;
            }
            if self.board[cy as usize][cx as usize] == 1 {
                return true;
            }
        }
        return false;
    }
    fn move_mino(&mut self, diff: Point) -> bool {
        let np = Point {
            y: self.position.y + diff.y,
            x: self.position.x + diff.x,
        };
        for i in 0..self.mino.block.shape.len() {
            let [y, x] = self.mino.block.shape[i];
            let ny = y + np.y;
            let nx = x + np.x;
            if self.is_out_of_range(ny, nx) {
                return false;
            }
        }
        // 削除
        for i in 0..self.mino.block.shape.len() {
            let [y, x] = self.mino.block.shape[i];
            let ny = y + self.position.y;
            let nx = x + self.position.x;
            self.board[ny as usize][nx as usize] = 0;
        }
        self.position = np;
        // 移動
        for i in 0..self.mino.block.shape.len() {
            let [y, x] = self.mino.block.shape[i];
            let ny = y + self.position.y;
            let nx = x + self.position.x;
            self.board[ny as usize][nx as usize] = 1;
        }
        return true;
    }
    pub fn fall(&mut self) -> bool {
        if !self.mino.is_falling {
            return false;
        }
        return self.move_mino(Point { y: 1, x: 0 });
    }
    // create new block at the top of the board
    pub fn spawn(&mut self) -> bool {
        let mino = Mino::new();
        self.reset_position();
        if self.is_conflict(&mino) {
            return false;
        }
        self.mino = mino;
        let shape = self.mino.block.shape;
        for i in 0..shape.len() {
            let [y, x] = shape[i];
            self.board[y as usize][x as usize] = 1;
        }
        return true;
    }
}
