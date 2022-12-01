use crate::{
    frame::{Drawable, Frame},
    {NUM_COLS, NUM_ROWS},
};
use rusty_time::timer::Timer;
use std::{cmp::max, time::Duration};

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
               if (x > 1)
               && (x < NUM_COLS - 2)
               && (y > 0)
               && (y < 9)
               && (x % 2 == 0)
               && (y % 2 == 0)
               {
               army.push(Invader { x, y }); 
            }
        }
    }
    Self {
        army,
        move_timer: Timer::from_millis(2000),
        direction: 1,
    }
}

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;
            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if max_x == NUM_COLS - 1 {
                self.direction = -1;
                downwards = true;
            }
        }
            return true;
        }
    }
    false
}