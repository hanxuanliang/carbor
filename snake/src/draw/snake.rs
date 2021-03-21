#![allow(dead_code)]

use piston_window::types::Color;
use piston_window::{Context, G2d};
use std::collections::LinkedList;

use crate::draw::action::draw_block;

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];

// 处理方向【蛇与键盘的交互运动】
#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    // 反转方向
    pub fn opposite(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

// 构建蛇的最小区块
#[derive(Clone)]
struct Block {
    x: i32,
    y: i32,
}

// 蛇：方向，{本体，尾巴}
#[derive(Clone)]
pub struct Snake {
    direction: Direction,
    body: LinkedList<Block>,
    tail: Option<Block>,
}

impl Snake {
    pub fn new(x: i32, y: i32) -> Snake {
        let mut body: LinkedList<Block> = LinkedList::new();
        // 初始化 [0 -> 1 -> 2]
        for i in (0..3).rev() {
            body.push_back(Block { x: x + i, y })
        }

        Snake {
            direction: Direction::Right,
            body,
            tail: None,
        }
    }

    // 画蛇本身
    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        for block in &self.body {
            draw_block(SNAKE_COLOR, block.x, block.y, ctx, g);
        }
    }

    // 返回蛇的头部
    pub fn head_pos(&self) -> (i32, i32) {
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    // 返回蛇当前的方向
    pub fn head_direction(&self) -> Direction {
        self.direction
    }

    // 蛇的前进
    pub fn move_forward(&mut self, dir: Option<Direction>) {
        match dir {
            Some(d) => self.direction = d,
            None => (),
        }

        let (last_x, last_y): (i32, i32) = self.head_pos();
        // 生成一个新区块 {👆 -> 因为在屏幕上的 y轴 是负的，所以要移动在人类角度是反向的}
        let new_block = match self.direction {
            Direction::Up => Block {
                x: last_x,
                y: last_y - 1,
            },
            Direction::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Direction::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Direction::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };
        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();
        self.tail = Some(removed_block);
    }

    pub fn next_head(&self, dir: Option<Direction>) -> (i32, i32) {
        let (head_x, head_y) = self.head_pos();

        let mut move_dir = self.direction;
        match dir {
            Some(d) => move_dir = d,
            None => {}
        }

        match move_dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
            Direction::Right => (head_x + 1, head_y),
        }
    }

    pub fn restore_tail(&mut self) {
        let block = self.tail.clone().unwrap();
        self.body.push_back(block);
    }

    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {
        let mut ch = 0;
        for block in &self.body {
            if x == block.x && y == block.y {
                return true;
            }

            ch += 1;
            if ch == self.body.len() - 1 {
                break;
            }
        }

        return false;
    }
}
