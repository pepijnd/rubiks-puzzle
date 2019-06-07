use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    R,
    G,
    B,
    Y,
}

#[derive(PartialEq, Eq)]
pub struct Piece {
    id: usize,
    rot: usize,
    shape: Vec<Color>,
}

impl From<&str> for Piece {
    fn from(string: &str) -> Piece {
        let mut shape = Vec::with_capacity(8);
        assert_eq!(
            string.len(),
            8,
            "piece should be 8 long is {}",
            string.len()
        );
        for c in string.chars() {
            match c {
                'R' => shape.push(Color::R),
                'G' => shape.push(Color::G),
                'B' => shape.push(Color::B),
                'Y' => shape.push(Color::Y),
                _ => panic!("unknown Color {}", c),
            }
        }
        Piece::new(shape)
    }
}

impl Piece {
    pub fn new(shape: Vec<Color>) -> Piece {
        Piece {
            id: 0,
            rot: 0,
            shape,
        }
    }

    fn get_sides(&self) -> Vec<&[Color]> {
        let mut sides = Vec::with_capacity(4);
        for n in 0..4 {
            sides.push(&self.shape[2 * n..2 * n + 2]);
        }
        sides
    }

    fn get_side(&self, side: usize) -> Vec<Color> {
        let n = side % 4;
        self.shape[2 * n..2 * n + 2].to_vec()
    }

    fn set_rot(&mut self, rot: usize) {
        self.rot = rot;
    }

    pub fn match_side(&self, other: &Piece, side: usize) -> bool {
        let lhs = self.get_side((self.rot + side) as usize);
        let mut rhs = other.get_side((other.rot + side + 2) as usize);
        rhs.reverse();
        lhs == rhs
    }

    fn match_side_rot(&self, other: &Piece, side: usize, rot: usize) -> bool {
        let lhs = self.get_side((4 + side - self.rot) as usize);
        let mut rhs = other.get_side((6 + side - rot) as usize);
        rhs.reverse();
        lhs == rhs
    }
}

pub struct Solver {
    options: Vec<Option<Vec<(usize, usize)>>>,
    puzzle: Puzzle,
    index_at: usize,
}

impl Solver {
    pub fn new(puzzle: Puzzle) -> Solver {
        Solver {
            options: vec![None; puzzle.width * puzzle.height],
            puzzle: puzzle,
            index_at: 0,
        }
    }

    pub fn solve(&mut self) -> bool {
        loop {
            if self.index_at == self.puzzle.width * self.puzzle.height {
                return true;
            }

            if self.options[self.index_at].is_none() {
                let options = self.puzzle.get_options(self.index_at);
                self.options[self.index_at] = Some(options);
            }
                    
            let options = &mut self.options[self.index_at].as_mut().unwrap();
            

            if options.len() == 0 {
                if self.index_at == 0 {
                    return false;
                }
                self.puzzle.set_none(self.index_at);
                self.options[self.index_at] = None;
                self.index_at -= 1;
                continue;
            }

            let (piece, rot) = options.pop().unwrap();
            self.puzzle.set_index(self.index_at, piece);
            self.puzzle.set_rot(piece, rot);

            self.index_at += 1;
        }
    }

    pub fn get_puzzle(&self) -> &Puzzle {
        &self.puzzle
    }
}

pub struct Puzzle {
    height: usize,
    width: usize,
    piece_list: HashMap<usize, Piece>,
    pieces: Vec<Option<usize>>,
}

impl Puzzle {
    pub fn new(width: usize, height: usize, pieces: Vec<Piece>) -> Puzzle {
        let mut piece_map = HashMap::new();
        for (i, mut piece) in pieces.into_iter().enumerate() {
            piece.id = i;
            piece_map.insert(i, piece);
        }

        Puzzle {
            width,
            height,
            piece_list: piece_map,
            pieces: vec![None; width * height],
        }
    }

    fn get_options(&self, index: usize) -> Vec<(usize, usize)> {
        let mut options = Vec::new();
        let (x, y) = self.get_pos(index);
        'piece: for piece in self.piece_list.values() {
            if self.pieces.contains(&Some(piece.id)) {
                continue 'piece;
            }
            'rot: for rot in 0..4 {
                if x == 0 && y == 0 {
                    options.push((piece.id, rot));
                    continue 'rot;
                }
                if x != 0 {
                    let idx = self.get_idx(x - 1, y);
                    let other = self.piece_list.get(&self.pieces[idx].unwrap()).unwrap();
                    if !other.match_side_rot(piece, 1, rot) {
                        continue 'rot;
                    }
                }
                if y != 0 {
                    let idx = self.get_idx(x, y - 1);
                    let other = self.piece_list.get(&self.pieces[idx].unwrap()).unwrap();
                    if !other.match_side_rot(piece, 2, rot) {
                        continue 'rot;
                    }
                }
                options.push((piece.id, rot));
            }
        }

        options
    }

    fn get_pos(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn get_idx(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    fn set_index(&mut self, index: usize, piece: usize) {
        self.pieces[index] = Some(piece);
    }

    fn set_none(&mut self, index: usize) {
        self.pieces[index] = None;
    }

    fn set_rot(&mut self, id: usize, rot: usize) {
        self.piece_list.get_mut(&id).unwrap().set_rot(rot);
    }
}

use std::fmt;

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}: {})", self.id+1, self.rot)
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = self.get_idx(x, y);
                write!(f, "{}\t", self.piece_list.get(&self.pieces[idx].unwrap()).unwrap())?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}