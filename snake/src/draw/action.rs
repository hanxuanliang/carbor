#![allow(dead_code)]

use piston_window::types::Color;
use piston_window::{rectangle, Context, G2d};

use crate::consts;

fn to_coord(coord: i32) -> f64 {
    (coord as f64) * consts::BLOCK_SIZE
}

// 画一个方格
pub fn draw_block(color: Color, x: i32, y: i32, ctx: &Context, g: &mut G2d) {
    let graph_x = to_coord(x);
    let graph_y = to_coord(y);

    rectangle(
        color,
        [graph_x, graph_y, consts::BLOCK_SIZE, consts::BLOCK_SIZE],
        ctx.transform,
        g,
    );
}

// 画一个矩形
pub fn draw_rectangle(
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    ctx: &Context,
    g: &mut G2d,
) {
    let rec_x = to_coord(x);
    let rec_y = to_coord(y);

    rectangle(
        color,
        [rec_x, rec_y, consts::BLOCK_SIZE * (width as f64), (height as f64) * consts::BLOCK_SIZE],
        ctx.transform,
        g,
    );
}
