use std::{usize};
use cursive::{Vec2};
use rand::Rng;


#[derive(Clone, Copy)]
pub struct Options {
    pub size: Vec2,
}

#[derive(Debug, PartialEq)]
#[derive(Clone, Copy)]
pub enum CellType {
    Food,
    Empty,
    Head,
    Tail,
    Body,
}

impl CellType {
    pub fn to_string(&self) -> &str {
        match self {
            CellType::Food => "O",

            CellType::Empty => ".",

            CellType::Head => "♦",
            CellType::Body => "#",
            CellType::Tail => "<",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MovementDirection {
    West,
    East,
    North,
    South,
    None,
}

#[derive(Debug, PartialEq)]
pub enum GameResult {
    Continue,
    WallCollision,
    SnakeCollision,
    Food,
}

#[derive(Clone, Debug)]
pub struct Snake {
    pub positions: Vec<usize>,
    direction: MovementDirection,
}

impl Snake {
    pub fn get_head_position(&self) -> usize {
        return self.positions[0];
    }

    pub fn get_tail_position(&self) -> usize {
        return self.positions[self.positions.len() - 1];
    }

    pub fn get_body_positions(&self) -> Vec<usize> {
        let mut body_positions: Vec<usize> = vec![];
        for index in 1..self.positions.len() - 1 {
            body_positions.push(self.positions[index]);
        }

        return body_positions;
    }

    pub fn move_east(&mut self, food_position: usize, max_x: usize) -> GameResult {
        let new_position = (self.get_head_position() % max_x) + 1;

        // Wall collision
        if new_position > max_x {
            return GameResult::WallCollision;
        }

        return self.move_and_check(self.get_head_position() + 1, food_position, MovementDirection::East);
    }

    pub fn move_west(&mut self, food_position: usize, max_x: usize) -> GameResult {
        let new_position: isize = ((self.get_head_position() % max_x) - 1) as isize;

        // Wall collision
        if new_position < 0 {
            return GameResult::WallCollision;
        }

        return self.move_and_check(self.get_head_position() - 1, food_position, MovementDirection::West);
    }

    pub fn move_south(&mut self, food_position: usize, max_x: usize, max_y: usize) -> GameResult {
        let new_position = self.get_head_position() + max_x;

        // Wall collision
        if new_position / max_x > max_y {
            return GameResult::WallCollision;
        }

        return self.move_and_check(new_position, food_position, MovementDirection::South);
    }

    pub fn move_north(&mut self, food_position: usize, max_x: usize) -> GameResult {
        let new_position:isize = (self.get_head_position() - max_x) as isize;

        // Wall collision
        if new_position < 0 {
            return GameResult::WallCollision;
        }

        return self.move_and_check(new_position as usize, food_position, MovementDirection::North);
    }


    fn move_and_check(&mut self, new_position: usize, food_position: usize, direction: MovementDirection) -> GameResult {
        // self collision
        if self.positions.contains(&new_position) {
            return GameResult::SnakeCollision;
        }

        // Normal Case
        let mut game_result = GameResult::Food;
        if new_position != food_position {
            self.positions.pop();
            game_result = GameResult::Continue;
        }

        // Advance head
        self.positions.insert(0, new_position);
        self.direction = direction;
        return game_result;
    }
}

#[derive(Debug)]
pub struct Board {
    pub cells: Vec<CellType>,
    pub size: Vec2,
    pub snake: Snake,
    pub(crate) food_position: usize,
}

impl Board {
    pub fn new(options: Options) -> Self {
        let size = options.size;
        let n_cells = size.x * size.y;

        // Snake
        let snake_index = n_cells / 2 + 8;
        let mut snake_positions: Vec<usize> = vec![Default::default(); 4];
        snake_positions[0] = snake_index;
        snake_positions[1] = snake_index - 1;
        snake_positions[2] = snake_index - 2;
        snake_positions[3] = snake_index - 3;

        let snake = Snake { positions: snake_positions, direction: MovementDirection::East };
        let mut board: Board = Self { cells: vec![CellType::Empty; n_cells], size, snake, food_position: 15 };
        board.add_snake();
        board.handle_food();
        return board;
    }

    fn redraw(&mut self) {
        let n_cells = self.size.x * self.size.y;
        self.cells = vec![CellType::Empty; n_cells];
        self.cells[self.food_position] = CellType::Food;
        self.add_snake();
    }

    fn add_snake(&mut self) {
        self.cells[self.snake.get_head_position()] = CellType::Head;
        self.cells[self.snake.get_tail_position()] = CellType::Tail;

        // Add body
        for position in self.snake.get_body_positions() {
            self.cells[position] = CellType::Body;
        }
    }

    fn get_new_food_position(&self) -> usize {
        let mut non_snake: Vec<usize> = vec![];

        for (index, cell) in self.cells.iter().enumerate() {
            match cell {
                CellType::Empty => { non_snake.push(index); }
                _ => { continue; }
            }
        }


        let random_index: usize = rand::thread_rng().gen_range(1..non_snake.len() - 1);
        return non_snake[random_index];
    }
    fn handle_food(&mut self) {
        let new_food_position = self.get_new_food_position();
        self.food_position = new_food_position;
        self.cells[new_food_position] = CellType::Food;
    }

    pub(crate) fn move_forward(&mut self, moved_direction: MovementDirection) -> GameResult {
        let game_result = match moved_direction {
            // Nothing pressed
            MovementDirection::None => {
                match self.snake.direction {
                    MovementDirection::East => {
                        self.snake.move_east(self.food_position, self.size.x)
                    }
                    MovementDirection::West => {
                        self.snake.move_west(self.food_position, self.size.x)
                    }
                    MovementDirection::North => {
                        self.snake.move_north(self.food_position, self.size.x)
                    }
                    MovementDirection::South => {
                        self.snake.move_south(self.food_position, self.size.x, self.size.y)
                    }
                    _ => { GameResult::Continue }
                }
            }
            // Pressing left
            MovementDirection::West => {
                match self.snake.direction {
                    MovementDirection::East => {
                        self.snake.move_east(self.food_position, self.size.x)
                    }
                    MovementDirection::West => {
                        self.snake.move_west(self.food_position, self.size.x)
                    }
                    MovementDirection::North => {
                        self.snake.move_west(self.food_position, self.size.x)
                    }
                    MovementDirection::South => {
                        self.snake.move_west(self.food_position, self.size.x)
                    }
                    _ => { GameResult::Continue }
                }
            }
            // Pressing right
            MovementDirection::East => {
                match self.snake.direction {
                    MovementDirection::East => {
                        self.snake.move_east(self.food_position, self.size.x)
                    }
                    MovementDirection::West => {
                        self.snake.move_west(self.food_position, self.size.x)
                    }
                    MovementDirection::North => {
                        self.snake.move_east(self.food_position, self.size.x)
                    }
                    MovementDirection::South => {
                        self.snake.move_east(self.food_position, self.size.x)
                    }
                    _ => { GameResult::Continue }
                }
            }
            // Pressing up
            MovementDirection::North => {
                match self.snake.direction {
                    MovementDirection::East => {
                        self.snake.move_north(self.food_position, self.size.x)
                    }
                    MovementDirection::West => {
                        self.snake.move_north(self.food_position, self.size.x)
                    }
                    MovementDirection::North => {
                        self.snake.move_north(self.food_position, self.size.x)
                    }
                    MovementDirection::South => {
                        self.snake.move_south(self.food_position, self.size.x, self.size.y)
                    }
                    _ => { GameResult::Continue }
                }
            }
            // Pressing down
            MovementDirection::South => {
                match self.snake.direction {
                    MovementDirection::East => {
                        self.snake.move_south(self.food_position, self.size.x, self.size.y)
                    }
                    MovementDirection::West => {
                        self.snake.move_south(self.food_position, self.size.x, self.size.y)
                    }
                    MovementDirection::North => {
                        self.snake.move_north(self.food_position, self.size.x)
                    }
                    MovementDirection::South => {
                        self.snake.move_south(self.food_position, self.size.x, self.size.y)
                    }
                    _ => { GameResult::Continue }
                }
            }
        };

        match game_result {
            GameResult::SnakeCollision => {
                return game_result;
            }
            GameResult::WallCollision => {
                return game_result;
            }
            GameResult::Food => {
                self.handle_food();
            }
            _ => {}
        }


        self.redraw();
        return game_result;
    }
}