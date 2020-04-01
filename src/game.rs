use crate::piece::*;

pub const BOARD_SIZE_X: usize = 10;
pub const BOARD_SIZE_Y: usize = 20;

pub struct Game {
    board: [[i8; BOARD_SIZE_X]; BOARD_SIZE_Y],
    piece: Piece,
    next_piece: Piece,
}

fn full(a: [i8; BOARD_SIZE_X])-> bool {
    let mut ret = true;
    
    for i in 0..BOARD_SIZE_X {
        if a[i] < 0 {
            ret = false;
        }
    }
    ret
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [[-1; BOARD_SIZE_X]; BOARD_SIZE_Y],
            piece: Piece::new(),
            next_piece: Piece::new(),
        }
    }

    pub fn draw_piece(&mut self) {
        let (px, py) = self.piece.get_position();

        for x in 0..PIECE_SIZE {
            for y in 0..PIECE_SIZE {
                let dx = x as i8 + px;
                let dy = y as i8 + py;
                let block = self.piece.get_block_at(x, y);
                if block >= 0{
                    if (dx >= 0) & (dy >= 0) & (dy < BOARD_SIZE_Y as i8) {
                        self.board[dy as usize][dx as usize] = self.piece.get_color() as i8;
                    }
                }                
            }
        }
    }

    pub fn clean_piece(&mut self) {
        let (px, py) = self.piece.get_position();

        for x in 0..PIECE_SIZE {
            for y in 0..PIECE_SIZE {
                let dx = x as i8 + px;
                let dy = y as i8 + py;
                let block = self.piece.get_block_at(x, y);
                if block >= 0{
                    if (dx >= 0) & (dy >= 0) & (dy < BOARD_SIZE_Y as i8) {
                        self.board[dy as usize][dx as usize] = -1;
                    }
                }                
            }
        }
    }

    pub fn lost(&self)-> bool {
        let mut ret = false;

        for i in 0..BOARD_SIZE_X {
            if self.board[0][i] >= 0 {
                ret = true
            }
        }

        ret
    }

    pub fn update_piece(&mut self)-> bool {
        self.clean();
        self.piece = self.next_piece;
        self.next_piece = Piece::new();
        self.lost()
    }

    pub fn change_piece(&mut self) {
        self.clean_piece();
        let aux = self.piece;
        self.piece = self.next_piece;
        self.next_piece = aux;
        self.draw_piece();
    }

    pub fn legal_position(&self)-> bool {
        let (px, py) = self.piece.get_position();
        let mut ret: bool = true;

        for x in 0..PIECE_SIZE {
            for y in 0..PIECE_SIZE {
                let dx = x as i8 + px;
                let dy = y as i8 + py;
                let block = self.piece.get_block_at(x, y);
                if block == 1{
                    if (dy >= BOARD_SIZE_Y as i8) | (dx >= BOARD_SIZE_X as i8) | (dx < 0) {
                        ret = false;
                    } else if (dx >= 0) & (dy >= 0) {
                        if self.board[dy as usize][dx as usize] >= 0 {
                            ret = false;
                        }
                    }
                }                
            }
        }

        ret
    }

    pub fn avance(&mut self)-> bool {
        let mut ret = true;
        self.draw_piece();
        self.print();
        self.clean_piece();
        self.piece.move_down();
        if !self.legal_position() {
            self.piece.move_up();
            self.draw_piece();
            ret = !self.update_piece();
        }
        ret
    }

    pub fn move_left(&mut self) {
        self.clean_piece();
        self.piece.move_left();
        if !self.legal_position() {
            self.piece.move_right();
            self.draw_piece();
        }
    }

    pub fn move_down(&mut self) {
        self.clean_piece();
        self.piece.move_down();
        if !self.legal_position() {
            self.piece.move_up();
            self.draw_piece();
        }
        
    }

    pub fn move_right(&mut self) {
        self.clean_piece();
        self.piece.move_right();
        if !self.legal_position() {
            self.piece.move_left();
            self.draw_piece();
        }
    }

    pub fn rotate(&mut self) {
        self.clean_piece();
        self.piece.rotate();
        self.print();
    }

    pub fn print(&self) {
        for y in self.board.iter() {
            for x in y {
                print!("{}", x);
            }
            print!("\n");
        }
    }

    pub fn pull_down(&mut self, p: usize) {  
        self.board[p] = [-1; BOARD_SIZE_X];      
        for i in (0..p + 1).rev() {
            if i != 0 {
                self.board[i] = self.board[i - 1];
            }
        }
    }

    pub fn clean(&mut self) {
        for i in 0..BOARD_SIZE_Y {
            if full(self.board[i]){
                self.pull_down(i);
            }
        }
    }

    

    pub fn get_board(&self)-> [[i8; BOARD_SIZE_X]; BOARD_SIZE_Y] {
        self.board
    }
}
