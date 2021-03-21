#![allow(dead_code)]

use piston_window::types::Color;
use piston_window::*;

use crate::draw::action::{draw_block, draw_rectangle};
use crate::draw::snake::{Direction, Snake};

use rand::{thread_rng, Rng};

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const BOARDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,
    food_exist: bool,
    food_x: i32,
    food_y: i32,
    width: i32,
    height: i32,
    game_over: bool,
    waiting_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            food_exist: true,
            food_x: 6,
            food_y: 4,
            width,
            height,
            game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => None,
        };

        // unwrap 如果出错了，
        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    // 游戏界面
    pub fn draw(&self, ctx: &Context, g: &mut G2d) {
        // 先画 蛇
        self.snake.draw(ctx, g);

        // 再画 food
        if self.food_exist {
            draw_block(FOOD_COLOR, self.food_x, self.food_y, ctx, g);
        }

        // 画边框
        draw_rectangle(BOARDER_COLOR, 0, 0, self.width, 1, ctx, g);
        draw_rectangle(BOARDER_COLOR, 0, self.height - 1, self.width, 1, ctx, g);
        draw_rectangle(BOARDER_COLOR, 0, 0, 1, self.height, ctx, g);
        draw_rectangle(BOARDER_COLOR, self.width - 1, 0, 1, self.height, ctx, g);

        // 如果游戏结束，画一个最大区域的红色
        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, ctx, g);
        }
    }

    // 更新目前的空间局面
    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        // 如果没有 food，就加一个 food
        if !self.food_exist {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }
    }

    // 吃掉food
    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_pos();
        // 头部与 food 重合，就说明，可以把它吃掉
        if self.food_exist && self.food_x == head_x && self.food_y == head_y {
            self.food_exist = false;
            self.snake.restore_tail();
        }
    }

    // 传入一个 direction，看蛇是否还活着，主要是在不在生存空间里面
    pub fn check_if_alive(&self, dir: Option<Direction>) -> bool {
        // 获取下一个蛇到达的点位
        let (next_x, next_y) = self.snake.next_head(dir);
        // 自己咬自己，肯定得dead
        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        // 看是否在空间内，是否撞墙
        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    // 加一个随机的果实
    pub fn add_food(&mut self) {
        let mut rng = thread_rng();

        // 产生一个区域范围在 [1, width-1] 的块
        let mut new_x = rng.gen_range(1, self.width - 1);
        let mut new_y = rng.gen_range(1, self.height - 1);
        // 如果和蛇重复，就需要一直重试产生一个不重复的 food点
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1, self.width - 1);
            new_y = rng.gen_range(1, self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exist = true;
    }

    // 更新蛇
    pub fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }

        self.waiting_time = 0.0;
    }

    // 重启游戏
    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exist = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}
