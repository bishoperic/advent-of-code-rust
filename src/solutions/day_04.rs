use crate::common::Solution;

pub struct Day04 {}

impl Solution for Day04 {
    fn part_a(&self, input: String) -> String {
        let mut input = input.lines();
        let instructions = input.next().unwrap();
        input.next();

        let mut boards = vec![];
        let mut board = vec![];
        for line in input {
            if line.is_empty() {
                boards.push(board.clone());
                board.clear();
                continue;
            }

            board.push(line);
        }

        let mut boards: Vec<BingoBoard> = boards
            .iter()
            .map(|x| x.join("\n"))
            .map(|x| BingoBoard::new(x))
            .collect();

        let mut bingo_board = BingoBoard::new("".to_string());
        let mut winning_instruction: isize = -1;
        'instructions: for num in instructions.split(",") {
            let num = num.parse().unwrap();
            for board in &mut boards {
                board.activate_num(num);
            }
            for board in &boards {
                if board.check_for_bingo() {
                    bingo_board = board.clone();
                    winning_instruction = num as isize;
                    break 'instructions;
                }
            }
        }

        if winning_instruction == -1 {
            panic!("Couldn't find a winning board!")
        }

        let mut sum = 0;
        for row in bingo_board.board {
            for space in row {
                if !space.1 {
                    sum += space.0;
                }
            }
        }

        (winning_instruction * sum as isize).to_string()
    }
    fn part_b(&self, input: String) -> String {
        let mut input = input.lines();
        let instructions = input.next().unwrap();
        input.next();

        let mut boards = vec![];
        let mut board = vec![];
        for line in input {
            if line.is_empty() {
                boards.push(board.clone());
                board.clear();
                continue;
            }

            board.push(line);
        }

        let mut boards: Vec<BingoBoard> = boards
            .iter()
            .map(|x| x.join("\n"))
            .map(|x| BingoBoard::new(x))
            .collect();

        let mut bingo_board: Option<BingoBoard> = None;
        let mut winning_instruction: Option<usize> = None;
        for num in instructions.split(",") {
            let num = num.parse().unwrap();
            for board in &mut boards {
                board.activate_num(num);
            }
            for board in &mut boards {
                if !board.won && board.check_for_bingo() {
                    board.won = true;
                    bingo_board = Some(board.clone());
                    winning_instruction = Some(num);
                }
            }
        }

        if let None = winning_instruction {
            panic!("Couldn't find a winning board!")
        }

        let mut sum = 0;
        for row in bingo_board.unwrap().board {
            for space in row {
                if !space.1 {
                    sum += space.0;
                }
            }
        }

        (winning_instruction.unwrap() * sum).to_string()
    }
}

struct BingoBoard {
    board: Vec<Vec<(usize, bool)>>,
    won: bool,
}

impl Clone for BingoBoard {
    fn clone(&self) -> Self {
        BingoBoard {
            board: self.board.clone(),
            won: false,
        }
    }
}

impl BingoBoard {
    fn new(input: String) -> BingoBoard {
        let mut new_board = BingoBoard {
            board: vec![vec![]],
            won: false,
        };
        new_board.set_board(input);

        new_board
    }

    fn set_board(&mut self, input: String) {
        let mut new_board = vec![];
        for row in input.lines() {
            new_board.push(
                row.split_whitespace()
                    .map(|x| (x.parse::<usize>().unwrap(), false))
                    .collect::<Vec<(usize, bool)>>(),
            );
        }

        if !self.check_for_square() {
            panic!("The board is not square!");
        }

        self.board = new_board;
    }

    fn activate_num(&mut self, num: usize) -> bool {
        for row in &mut self.board {
            if let Some(column) = row.iter().position(|space| space.0 == num) {
                row[column].1 = true;
                return true;
            }
        }

        false
    }

    fn check_for_bingo(&self) -> bool {
        let bingo_length = 5;
        let size = self.dimensions();

        let mut horizontals = vec![];
        for row in &self.board {
            horizontals.push(row.to_owned());
        }

        let mut verticals = vec![];
        for column in 0..size.width {
            let mut new_row = vec![];
            for row in &self.board {
                new_row.push(row[column]);
            }
            verticals.push(new_row);
        }

        let mut up_diagonals = self.diagonals(true);
        let mut down_diagonals = self.diagonals(false);

        let mut directions: Vec<Vec<(usize, bool)>> = Vec::new();
        directions.append(&mut horizontals);
        directions.append(&mut verticals);
        directions.append(&mut up_diagonals);
        directions.append(&mut down_diagonals);

        let mut bingo_count = 0;
        for slice in directions {
            let mut count = 0;

            for space in slice {
                if space.1 {
                    count += 1;
                } else {
                    count = 0;
                }

                if count >= bingo_length {
                    bingo_count += 1;
                    break;
                }
            }
        }

        bingo_count > 0
    }

    fn diagonals(&self, up: bool) -> Vec<Vec<(usize, bool)>> {
        let size = self.dimensions();
        let mut starting_pos = vec![];

        // set the starting positions to go around the edge of the board
        if up {
            for i in 0..size.height {
                starting_pos.push(i * size.width);
            }
            for i in 1..size.width {
                starting_pos.push(size.width * size.height - size.width + i);
            }
        } else {
            for i in (0..size.height).rev() {
                starting_pos.push(i * size.width);
            }
            for i in 1..size.width {
                starting_pos.push(i);
            }
        }

        let mut diagonals = vec![];
        for pos in starting_pos {
            let mut pos: isize = pos as isize;
            let mut new_diagonal = vec![];
            while match up {
                true => pos > 0,
                false => pos < (size.width * size.height) as isize,
            } {
                new_diagonal.push(self.board[pos as usize / size.width][pos as usize % size.width]);
                // check if position is on the right edge
                if (pos + 1) as usize % size.width == 0 {
                    break;
                }
                if up {
                    pos -= size.width as isize - 1;
                } else {
                    pos += size.width as isize + 1;
                }
            }
            diagonals.push(new_diagonal);
        }

        diagonals
    }

    fn check_for_square(&self) -> bool {
        let len = self.board.first().unwrap().len();
        for row in &self.board {
            if row.len() != len {
                return false;
            }
        }
        true
    }

    fn dimensions(&self) -> Dimensions {
        let height = self.board.len();
        let width = self.board[0].len();
        Dimensions { height, width }
    }
}

struct Dimensions {
    height: usize,
    width: usize,
}
