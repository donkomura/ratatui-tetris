use rand::Rng;

#[derive(Clone, PartialEq, Debug)]
pub enum TETROMINOS {
    // all mino consists of 4 blocks
    STRAIGHT,
    SQUARE,
    T,
    L,
    S,
}

#[derive(Clone, Debug)]
pub struct Shape {
    pub kind: TETROMINOS,
    pub points: [[i32; 2]; 4], // [y, x]
}

impl Shape {
    fn straight() -> Self {
        Shape {
            kind: TETROMINOS::STRAIGHT,
            points: [[0, 0], [0, 1], [0, 2], [0, 3]],
        }
    }
    fn square() -> Self {
        Shape {
            kind: TETROMINOS::SQUARE,
            points: [[0, 0], [0, 1], [1, 0], [1, 1]],
        }
    }
    fn t() -> Self {
        Shape {
            kind: TETROMINOS::T,
            points: [[0, 0], [0, 1], [0, 2], [1, 1]],
        }
    }
    fn l() -> Self {
        Shape {
            kind: TETROMINOS::L,
            points: [[0, 0], [0, 1], [0, 2], [1, 2]],
        }
    }
    fn s() -> Self {
        Shape {
            kind: TETROMINOS::S,
            points: [[0, 0], [0, 1], [1, 1], [1, 2]],
        }
    }
    pub fn new(kind: TETROMINOS) -> Self {
        match kind {
            TETROMINOS::STRAIGHT => Shape::straight(),
            TETROMINOS::SQUARE => Shape::square(),
            TETROMINOS::T => Shape::t(),
            TETROMINOS::L => Shape::l(),
            TETROMINOS::S => Shape::s(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mino {
    pub is_falling: bool,
    pub block: Shape, // shape of the tetromino
}

impl Mino {
    pub fn new() -> Self {
        const TETRAMINOS_LEN: u32 = 5;
        let kind = match rand::thread_rng().gen_range(0..TETRAMINOS_LEN) {
            0 => TETROMINOS::STRAIGHT,
            1 => TETROMINOS::SQUARE,
            2 => TETROMINOS::T,
            3 => TETROMINOS::L,
            4 => TETROMINOS::S,
            _ => TETROMINOS::STRAIGHT,
        };
        Mino {
            is_falling: false,
            block: Shape::new(kind),
        }
    }
    pub fn rotate_left(&mut self) {
        let block = self.block.clone();
        if block.kind == TETROMINOS::SQUARE {
            return;
        }
        let mut new_points = [[0; 2]; 4];
        for i in 0..block.points.len() {
            let [y, x] = block.points[i];
            new_points[i] = [-x, y];
        }
        self.block.points = new_points;
    }
    pub fn rotate_right(&mut self) {
        let block = self.block.clone();
        if block.kind == TETROMINOS::SQUARE {
            return;
        }
        let mut new_points = [[0; 2]; 4];
        for i in 0..block.points.len() {
            let [y, x] = block.points[i];
            new_points[i] = [x, -y];
        }
        self.block.points = new_points;
    }
}

#[derive(Clone, Debug)]
pub struct Point {
    pub y: i32,
    pub x: i32,
}

#[derive(Debug)]
pub struct App {
    width: u16,
    height: u16,
    pub score: u64,
    pub running: bool,
    pub mino: Mino,
    pub position: Point,        // left corner of the tetromino
    pub board: [[i32; 10]; 20], // 20x10 board
}

const BOARD_WIDTH: u16 = 10;
const BOARD_HEIGHT: u16 = 20;

impl Default for App {
    fn default() -> Self {
        App {
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
            score: 0,
            running: true,
            mino: Mino::new(),
            position: Point { y: 0, x: 0 },
            board: [[0; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
        }
    }
}

impl App {
    pub fn new() -> App {
        Self::default()
    }
    pub fn quiet(&mut self) {
        self.running = false;
    }
    pub fn reset_position(&mut self) {
        self.position = Point { y: 0, x: 0 };
    }
    pub fn check_state(&mut self) {
        if !self.fall() {
            self.mino.is_falling = false;
        }
        if !self.mino.is_falling {
            if !self.spawn() {
                self.quiet();
            }
            self.mino.is_falling = true;
        }
    }
    pub fn fall(&mut self) -> bool {
        if !self.mino.is_falling {
            return false;
        }
        return self.move_mino(&Point { y: 1, x: 0 });
    }
    // create new block at the top of the board
    pub fn spawn(&mut self) -> bool {
        let mino = Mino::new();
        self.reset_position();
        if self.is_conflict(&self.position, &mino) {
            return false;
        }
        self.render(&mino, &self.position.clone(), 1);
        self.mino = mino;
        return true;
    }
    pub fn width(&self) -> u16 {
        return self.width;
    }
    pub fn height(&self) -> u16 {
        return self.height;
    }
    pub fn move_right(&mut self) {
        self.move_mino(&Point { y: 0, x: 1 });
    }
    pub fn move_left(&mut self) {
        self.move_mino(&Point { y: 0, x: -1 });
    }
    pub fn move_down(&mut self) {
        self.move_mino(&Point { y: 1, x: 0 });
    }
    pub fn rotate(&mut self) {
        let mut mino = self.mino.clone();
        let position = self.position.clone();
        self.render(&mut mino, &position, 0); // Borrow `self.mino` as mutable
        mino.rotate_right();
        if self.is_conflict(&position, &mino) {
            mino.rotate_left();
        }
        self.mino = mino;
        self.move_down();
    }
    fn is_out_of_range(&self, py: i32, px: i32) -> bool {
        if py < 0 || py >= 20 || px < 0 || px >= 10 {
            return true;
        }
        return false;
    }
    fn is_conflict(&self, position: &Point, mino: &Mino) -> bool {
        let points = &mino.block.points;
        for i in 0..points.len() {
            let [y, x] = points[i];
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
    fn render(&mut self, mino: &Mino, base: &Point, value: i32) {
        for i in 0..mino.block.points.len() {
            let [y, x] = mino.block.points[i];
            let ny = y + base.y;
            let nx = x + base.x;
            self.board[ny as usize][nx as usize] = value;
        }
    }
    fn move_mino(&mut self, diff: &Point) -> bool {
        let np = Point {
            y: self.position.y + diff.y,
            x: self.position.x + diff.x,
        };
        // 移動前の描画を消す
        let mino = self.mino.clone();
        let base = self.position.clone();
        self.render(&mino, &base, 0);
        // 移動先で衝突検査
        if self.is_conflict(&np, &mino) {
            self.render(&mino, &base, 1);
            return false;
        }
        // 移動
        self.render(&mino, &np, 1);
        self.position = np;
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    trait FactoryForTest {
        fn create(kind: TETROMINOS) -> Self;
    }
    impl FactoryForTest for Mino {
        fn create(kind: TETROMINOS) -> Self {
            Mino {
                is_falling: false,
                block: Shape::new(kind),
            }
        }
    }

    #[test]
    fn rotate_round() {
        let mut mino = Mino::create(TETROMINOS::STRAIGHT);
        let original = mino.clone();
        mino.rotate_left();
        assert_eq!(mino.block.points, [[0, 0], [-1, 0], [-2, 0], [-3, 0]]);
        mino.rotate_left();
        assert_eq!(mino.block.points, [[0, 0], [0, -1], [0, -2], [0, -3]]);
        mino.rotate_left();
        assert_eq!(mino.block.points, [[0, 0], [1, 0], [2, 0], [3, 0]]);
        mino.rotate_left();
        assert_eq!(mino.block.points, original.block.points);
    }
    #[test]
    fn rotate_equivalence() {
        let mut mino = Mino::create(TETROMINOS::STRAIGHT);
        let original = mino.clone();
        mino.rotate_left();
        mino.rotate_right();
        assert_eq!(mino.block.points, original.block.points);
        mino.rotate_right();
        mino.rotate_left();
        assert_eq!(mino.block.points, original.block.points);
    }
}
