use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK: f64 = 20.0;

pub fn to_game_coordinate(game_coordinate: i32) -> f64{
    (game_coordinate as f64) * BLOCK
}

pub fn to_game_coordinate_u32(game_coordinate: i32) -> u32{
    to_game_coordinate(game_coordinate) as u32
}

pub fn draw_block(color: Color, x: i32, y:i32, context: &Context, graphics: &mut G2d){
    let x_cord = to_game_coordinate(x);
    let y_cord = to_game_coordinate(y);
    rectangle(color,[x_cord, y_cord, BLOCK, BLOCK ],
              context.transform, graphics);
    // rectangle(color, [x_cord, y_cord, BLOCK, BLOCK], context.transform, graphics);


    println!("DRAW BLOCK : {:?}, {}, {}, {}", color, x_cord, y_cord, BLOCK);
}

pub fn draw_rectangle(color: Color, x: i32, y: i32, width: i32, height: i32,
                      context: &Context, graphics: &mut G2d){
    let x_cord = to_game_coordinate(x);
    let y_cord = to_game_coordinate(y);

    println!("MAIN RECT BLOCK : {:?}, {}, {}, {}", color, x_cord, y_cord, BLOCK);
    rectangle(color,[x_cord, y_cord, BLOCK * (width as f64), BLOCK  * (width as f64)],
              context.transform, graphics);

}