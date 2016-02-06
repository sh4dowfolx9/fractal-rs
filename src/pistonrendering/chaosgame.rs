// Copyright (c) 2016 William (B.J.) Snow Orvis
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Window handlers for drawing points as part of playing a ChaosGame.

use std::sync::Arc;
use graphics;
use piston_window::*;

use super::super::chaosgame::{ChaosGame, ChaosGameMoveIterator};
use super::{BLACK, WHITE, WindowHandler, WhichFrame};
use super::super::geometry::Point;

/// Draw a dot at the given point. (0.0,0.0) is the center of the screen, (1.0,1.0) is near the top
/// right, and (-1.0,-1.0) is near the bottom left.
fn draw_dot(context: graphics::context::Context, gfx: &mut G2d, point: Point) {
    let view_size = context.get_view_size();
    let screen_width = view_size[0];
    let screen_height = view_size[1];

    let originx = screen_width / 2.0f64;
    let originy = screen_height / 2.0f64;

    let taller = (screen_height as i64 - screen_width as i64) > 0;

    // use the smaller direction to determine how many pixels are in one unit of
    // distance here.
    let one_unit_to_pixels = if taller {
        (screen_width / 2f64)
    } else {
        (screen_height / 2f64)
    };

    let transform = context.transform
                           .trans(originx, originy)
                           .zoom(one_unit_to_pixels)
                           .flip_v()
                           .trans(0.0, 0.0);

    let delta = 0.5 / one_unit_to_pixels as f64;

    // println!("Drawing {}", point);
    Rectangle::new(BLACK).draw([point.x - delta, point.y - delta, 2.0 * delta, 2.0 * delta],
                               default_draw_state(),
                               transform,
                               gfx);
}

pub struct ChaosGameWindowHandler {
    game: Arc<ChaosGame>,
    which_frame: WhichFrame,
    iter: Option<ChaosGameMoveIterator>,
    dots_per_frame: usize,
    last_moves: Vec<Point>,
}

impl ChaosGameWindowHandler {
    pub fn new(game: Arc<ChaosGame>, dots_per_frame: usize) -> ChaosGameWindowHandler {
        ChaosGameWindowHandler {
            game: game,
            which_frame: WhichFrame::FirstFrame,
            iter: None,
            dots_per_frame: dots_per_frame,
            last_moves: Vec::with_capacity(dots_per_frame),
        }
    }
}

impl WindowHandler for ChaosGameWindowHandler {
    fn window_resized(&mut self) {
        self.which_frame = WhichFrame::FirstFrame;
        self.iter = None;
        self.last_moves = Vec::with_capacity(self.dots_per_frame);
    }

    fn render_frame(&mut self, context: graphics::context::Context, gfx: &mut G2d, _: u32) {
        match self.which_frame {
            WhichFrame::FirstFrame => {
                // The first frame clears its screen and starts drawing.
                clear(WHITE, gfx);
                self.iter = Some(ChaosGameMoveIterator::new(self.game.clone()));
                // draw up to dots_per_frame dots, and store them for the next frame to also
                // draw
                for _ in 0..self.dots_per_frame {
                    if let Some(next_point) = self.iter.as_mut().unwrap().next() {
                        draw_dot(context, gfx, next_point);
                        self.last_moves.push(next_point);
                    }
                }
                self.which_frame = WhichFrame::SecondFrame;
            }
            WhichFrame::SecondFrame => {
                // The second frame is on the second buffer, so it needs to clear the screen,
                // draw the first frame's dots, and then draw some more dots.
                clear(WHITE, gfx);
                // catch up to the first frame by draining last_moves
                for oldmove in self.last_moves.drain(..) {
                    draw_dot(context, gfx, oldmove);
                }
                // draw up to dots_per_frame dots, and refill last_moves.
                for _ in 0..self.dots_per_frame {
                    if let Some(next_point) = self.iter.as_mut().unwrap().next() {
                        draw_dot(context, gfx, next_point);
                        self.last_moves.push(next_point);
                    }
                }
                self.which_frame = WhichFrame::AllOtherFrames;
            }
            _ => {
                // All remaining frames need to catch up to the last frame, and then move
                // forward.
                for oldmove in self.last_moves.drain(..) {
                    draw_dot(context, gfx, oldmove);
                }
                for _ in 0..self.dots_per_frame {
                    if let Some(next_point) = self.iter.as_mut().unwrap().next() {
                        draw_dot(context, gfx, next_point);
                        self.last_moves.push(next_point);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
}
