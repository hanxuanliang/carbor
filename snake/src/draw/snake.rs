use piston_window::types::Color;
use std::collections::LinkedList;
use piston_window::{ Context, G2d};

use crate::draw::action::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

enum Direction {
    Up, 
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

struct Block {
    x: i32,
    y: i32,
}

struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        // 初始化 [0 -> 1 -> 2]
        for i in (0..3).rev() {
            body.push_back(Block {
                x: x + i,
                y,
            })
        }

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    pub fn draw(&self, ctx: Context, g: &mut G2d) {
        for block in &self.body {
            
        }
    }
}
