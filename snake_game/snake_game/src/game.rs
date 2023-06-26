use std::collections::hash_map::Keys;
use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};
use crate::draw::{draw_block, draw_rectangle};
use crate::snake::{Direction, Snake};

const COLOR_BORDER: Color = [0.5, 0.5, 0.5, 1.0];
const FOOD_COLOR: Color = [0.8, 0.0, 0.0, 1.0];
const GAMEOVERCOLOR: Color = [0.8, 0.0, 0.0, 0.3];
const MOVEPERIOD: f64 = 0.2;
const RESTART: f64 = 1.0; //1 sec

pub struct Game{
    snake: Snake,
    food_exists: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,
    wait_time: f64,
    game_over: bool
}

impl Game {
    // constructor
    pub fn new(width: i32, height: i32) -> Game{
        Game{
            snake: Snake::new(4,4),
            food_exists: false,
            food_x: 0,
            food_y: 0,
            width,
            height,
            wait_time: 0.0,
            game_over: false
        }
    }


    pub fn keypressed(&mut self, whichkey: Key){
        if self.game_over{
            return;
        }

        let directionkey = match whichkey {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.headdirection()) //default
        };

        if directionkey.unwrap() == self.snake.headdirection().opposite_direction(){
            return;
        }
        self.update_snake(directionkey);
    }

    pub fn draw(&self, context: &Context, graphics: &mut G2d){
        self.snake.draw(context, graphics);
        if self.food_exists {
            println!("######## {}, {} ###########", self.food_x, self.food_y);
            draw_block(FOOD_COLOR, self.food_x, self.food_y, context, graphics);
        }
        if self.game_over{
            draw_rectangle(GAMEOVERCOLOR, 0, 0, self.width, self.height, context,
                           graphics);
        }
    }

    pub fn update(&mut self, delta_time: f64){
        self.wait_time += delta_time;

        if self.game_over{
            if self.wait_time > RESTART{
                self.restart();
            }
            return;
        }
        if !self.food_exists {
            println!("!self.food_exists");
            self.add_food();
        }

        if self.wait_time > MOVEPERIOD{
            self.update_snake(None);
        }
    }


    fn check_snake_alive(&self, direction:Option<Direction>)->bool{
        let (next_stepX, next_stepY) = self.snake.next_head(direction);
        if self.snake.overlaptail(next_stepX, next_stepY){
            return false;
        }
        //return if snake is inside box or not
        next_stepX > 0 && next_stepY > 0 && next_stepX < self.width-1 && next_stepY < self.height-1
    }

    fn check_eating(&mut self){
        let (head_x, head_y): (i32, i32) = self.snake.headposition();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restoretail();
        }
    }

    fn update_snake(&mut self, direction: Option<Direction>) {
        if self.check_snake_alive(direction) {
            self.snake.move_snake(direction);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.wait_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(3, 3);
        self.wait_time = 0.0;
        self.food_exists = false;
        self.food_x = 0;
        self.food_y = 0;
        self.game_over = false;
    }

    fn add_food(&mut self){
        let mut rng = thread_rng();
        let mut new_x =  rng.gen_range(1..self.width-1);
        let mut new_y = rng.gen_range(1..self.height-1);
        while self.snake.overlaptail(new_x, new_y) {
            new_x =  rng.gen_range(1..self.width-1);
            new_y = rng.gen_range(1..self.height-1);
        }
        println!("{}, {}", new_x, new_y);
        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }



}