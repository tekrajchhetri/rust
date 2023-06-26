use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;

use crate::draw::draw_block;

const SNAKE_COLOR: Color = [0.6, 0.5, 0.3, 1.0];

#[derive(Copy, Clone, PartialEq)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right
}

impl Direction{
    pub fn opposite_direction(&self) -> Direction{
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left

        }
    }
}

#[derive(Debug, Clone)]
struct Block{
    x: i32,
    y:i32
}

pub struct Snake{
    direction: Direction,
    snake_body: LinkedList<Block>,
    tail: Option<Block>
}

impl Snake{
    pub fn new(x:i32, y:i32)-> Snake{
        let mut snake_body:LinkedList<Block> = LinkedList::new();
        snake_body.push_back(Block{x: x+2, y: y,});
        snake_body.push_back(Block{x: x+1, y: y,});
        snake_body.push_back(Block{x: x, y: y,});

        Snake{
            direction: Direction::Right,
            snake_body: snake_body,
            tail: None,
        }
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d){
        for block in &self.snake_body {
            draw_block(SNAKE_COLOR, block.x, block.y, context, graphics);
        }
    }

    pub fn headposition(&self)-> (i32, i32){
        let headblock = self.snake_body.front().unwrap();
        //return
        (headblock.x, headblock.y)
    }

    pub fn move_snake(&mut self, direction: Option<Direction>){
        match direction {
            Some(d) => self.direction = d,
            None=>()
        }

        let (last_x, last_y): (i32,i32) = self.headposition();

        let new_block = match self.direction {
            Direction::Up => Block {x: last_x, y: last_y - 1},
            Direction::Down => Block {x: last_x, y: last_y + 1},
            Direction::Left => Block {x: last_x - 1, y: last_y},
            Direction::Right => Block {x: last_x + 1, y: last_y}
        };

        self.snake_body.push_front(new_block);
        let removed_block = self.snake_body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn headdirection(&self)->Direction{
        self.direction
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y): (i32, i32) = self.headposition();
        let mut movingdir = self.direction;
        match dir {
            Some(d) => movingdir = d,
            None => {}
        }

        match movingdir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y)
        }
    }

    pub fn restoretail(&mut self) {
        //clone the block and add it to back
        let blks = self.tail.clone().unwrap();
        self.snake_body.push_back(blks);
    }

    pub fn overlaptail(&self, x: i32, y: i32) -> bool {
        let mut c = 0;
        for block in &self.snake_body {
            if x == block.x && y == block.y {
                return true;
            }

            c += 1;
            if c == self.snake_body.len() - 1 {
                break;
            }
        }
        return false;
    }


}