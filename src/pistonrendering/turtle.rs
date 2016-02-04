// Copyright (c) 2015-2016 William (B.J.) Snow Orvis
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

//! Window handlers for drawing TurtlePrograms.

use graphics;
use piston_window::*;
use std::fmt;

use geometry::{Point, Vector};
use super::{BLACK, WHITE, WindowHandler, WhichFrame};
use turtle::{Turtle, TurtleProgram, TurtleCollectToNextForwardIterator};

pub fn construct_turtle_window_handler<'a>(program: &'a TurtleProgram,
                                           animate: u64)
                                           -> Box<WindowHandler + 'a> {
    match animate {
        0 => Box::new(DoubleBufferedWindowHandler::new(program)),
        _ => Box::new(DoubleBufferedAnimatedWindowHandler::new(program, animate)),
    }
}


/// Internal state of a turtle. Can be used by turtle implementations to store/pause their drawing.
#[derive(Clone, Debug)]
pub struct TurtleState {
    position: Point,
    angle: f64,
    down: bool,
}

impl TurtleState {
    /// Initializes a new TurtleState. A new turtle starts at (0,0) and faces towards the positive
    /// X axis.
    pub fn new() -> TurtleState {
        TurtleState {
            position: Point { x: 0.0, y: 0.0 },
            angle: 0.0,
            down: true,
        }
    }
}

/// An implementation of a Turtle within a Piston window/gfx context.
pub struct PistonTurtle<'a, G>
    where G: Graphics + 'a
{
    gfx: &'a mut G,
    /// Needed to calculate transformations from the turtle coordinate system to the window's
    /// coordinate system.
    window_size: Size,
    context: graphics::context::Context,

    state: &'a mut TurtleState,
}

impl<'a, G> PistonTurtle<'a, G> where G: Graphics + 'a
{
    pub fn new(state: &'a mut TurtleState,
               gfx: &'a mut G,
               window_size: Size,
               context: graphics::context::Context)
               -> PistonTurtle<'a, G> {
        PistonTurtle {
            gfx: gfx,
            window_size: window_size,
            context: context,
            state: state,
        }
    }
}

impl<'a, G> Turtle for PistonTurtle<'a, G> where G: Graphics + 'a
{
    fn forward(&mut self, distance: f64) {
        let old_pos = self.state.position;
        let new_pos = self.state.position.point_at(Vector {
            direction: self.state.angle,
            magnitude: distance,
        });

        if self.state.down {
            let screen_width = self.window_size.width;
            let screen_height = self.window_size.height;

            let startx = (screen_width / 4) as f64;
            let starty = (screen_height / 2) as f64;
            let endx = (3 * screen_width / 4) as f64;
            // let endy = (screen_height / 2) as f64;

            let linesize = (startx - endx).abs() as f64;

            let transform = self.context
                                .transform
                                .trans(startx, starty)
                                .zoom(linesize)
                                .flip_v()
                                .trans(0.0, 0.0);

            Line::new(BLACK, 0.5 / linesize).draw([old_pos.x, old_pos.y, new_pos.x, new_pos.y],
                                                  default_draw_state(),
                                                  transform,
                                                  self.gfx);
        }

        self.state.position = new_pos;
    }

    fn set_pos(&mut self, new_pos: Point) {
        self.state.position = new_pos;
    }

    fn set_rad(&mut self, new_rad: f64) {
        self.state.angle = new_rad;
    }

    fn turn_rad(&mut self, radians: f64) {
        use std::f64::consts::PI;
        self.state.angle = (self.state.angle + radians) % (2.0 * PI);
    }

    fn down(&mut self) {
        self.state.down = true;
    }

    fn up(&mut self) {
        self.state.down = false;
    }
}


/// WindowHandler that renders an entire turtle program per-frame, and optimizes re-renders
/// by only rendering twice (once for each buffer).
pub struct DoubleBufferedWindowHandler<'a> {
    program: &'a TurtleProgram,
    /// Whether we need to re-render for double-buffered frames.
    redraw: [bool; 2],
}

impl<'a> DoubleBufferedWindowHandler<'a> {
    pub fn new(program: &TurtleProgram) -> DoubleBufferedWindowHandler {
        DoubleBufferedWindowHandler {
            program: program,
            redraw: [true; 2],
        }
    }

    fn turtledraw(program: &TurtleProgram, turtle: &mut Turtle) {
        let init_turtle_steps = program.init_turtle();

        for action in init_turtle_steps {
            turtle.perform(action)
        }

        for action in program.turtle_program_iter() {
            turtle.perform(action)
        }
        turtle.up();
    }
}

impl<'a> fmt::Debug for DoubleBufferedWindowHandler<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "DoubleBufferedWindowHandler(program:<program>, redraw:{:?})",
               self.redraw)
    }
}

impl<'a> WindowHandler for DoubleBufferedWindowHandler<'a> {
    fn window_resized(&mut self) {
        self.redraw[0] = true;
        self.redraw[1] = true;
    }

    fn render_frame(&mut self,
                    window_size: Size,
                    context: graphics::context::Context,
                    gfx: &mut G2d,
                    frame_num: u32) {
        let redraw = self.redraw[(frame_num % 2) as usize];
        if redraw {
            println!("Redrawing frame {}", frame_num % 2);
            clear(WHITE, gfx);

            let mut state = TurtleState::new();
            let mut turtle = PistonTurtle::new(&mut state, gfx, window_size, context);
            DoubleBufferedWindowHandler::turtledraw(self.program, &mut turtle);

            println!("Done redrawing frame");
            self.redraw[(frame_num % 2) as usize] = false;
        }
    }
}

/// WindowHandler that animates the drawing of the curve by only adding a few line segments per
/// frame.
pub struct DoubleBufferedAnimatedWindowHandler<'a> {
    program: &'a TurtleProgram,
    /// stored turtle state for each turtle. double-buffered means we need to animate the curve
    /// "twice".
    turtles: [TurtleState; 2],
    /// Two iterators.
    iters: [TurtleCollectToNextForwardIterator<'a>; 2],
    lines_per_frame: u64,
    /// Which frame we are rendering. We need to perform the initial steps for the first frame, and
    /// we need perform the initial steps and do one extra move forward for the second frame (to
    /// stagger the double buffer). The rest of the frames then just move forward.
    which_frame: WhichFrame,
}

impl<'a> fmt::Debug for DoubleBufferedAnimatedWindowHandler<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "DoubleBufferedAnimatedWindowHandler(turtles:{:?}, iters:<iters>, which_frame:{:?})",
               self.turtles,
               self.which_frame)
    }
}

impl<'a> DoubleBufferedAnimatedWindowHandler<'a> {
    /// Initialize a new DoubleBufferedAnimatedWindowHandler.
    ///
    /// `lines_per_frame` specifies how many line segments the turtle should draw per frame.
    pub fn new(program: &'a TurtleProgram,
               lines_per_frame: u64)
               -> DoubleBufferedAnimatedWindowHandler<'a> {
        DoubleBufferedAnimatedWindowHandler {
            program: program,
            turtles: [TurtleState::new(), TurtleState::new()],
            iters: [TurtleCollectToNextForwardIterator::new_null_iter(),
                    TurtleCollectToNextForwardIterator::new_null_iter()],
            lines_per_frame: lines_per_frame,
            which_frame: WhichFrame::FirstFrame,
        }
    }

    fn draw_one_move<G>(turtle: &mut PistonTurtle<G>,
                        program_iter: &mut TurtleCollectToNextForwardIterator)
        where G: Graphics
    {
        let one_move = program_iter.next();
        if !one_move.is_none() {
            for action in one_move.unwrap() {
                turtle.perform(action)
            }
        }
    }
}

impl<'a> WindowHandler for DoubleBufferedAnimatedWindowHandler<'a> {
    fn window_resized(&mut self) {
        self.which_frame = WhichFrame::FirstFrame;
        self.turtles[0] = TurtleState::new();
        self.turtles[1] = TurtleState::new();
    }

    fn render_frame(&mut self,
                    window_size: Size,
                    context: graphics::context::Context,
                    gfx: &mut G2d,
                    frame_num: u32) {
        let bufnum = (frame_num % 2) as usize;

        match self.which_frame {
            WhichFrame::FirstFrame => {
                // gfx can only be &mut borrowed by one thing at a time. If we loan it to the
                // turtle and also use it elsewhere, this would trigger the static analysis.
                // This could be worked around by placing gfx into a RefCell.
                clear(WHITE, gfx);
                let mut turtle = PistonTurtle::new(&mut self.turtles[bufnum],
                                                   gfx,
                                                   window_size,
                                                   context);
                for action in self.program.init_turtle() {
                    turtle.perform(action)
                }
                self.iters[bufnum] = self.program.turtle_program_iter().collect_to_next_forward();
                self.which_frame = WhichFrame::SecondFrame;
            }
            WhichFrame::SecondFrame => {
                clear(WHITE, gfx);
                let mut turtle = PistonTurtle::new(&mut self.turtles[bufnum],
                                                   gfx,
                                                   window_size,
                                                   context);
                for action in self.program.init_turtle() {
                    turtle.perform(action)
                }
                self.iters[bufnum] = self.program.turtle_program_iter().collect_to_next_forward();
                // if we are the second frame, then we need to stagger our buffer from
                // the first buffer.
                let one_move = self.iters[bufnum].next();
                if !one_move.is_none() {
                    for action in one_move.unwrap() {
                        turtle.perform(action)
                    }
                }
                self.which_frame = WhichFrame::AllOtherFrames;
            }
            _ => {
                let mut turtle = PistonTurtle::new(&mut self.turtles[bufnum],
                                                   gfx,
                                                   window_size,
                                                   context);
                for _ in 0..self.lines_per_frame {
                    // Make 2 moves per frame since we are double buffered.
                    DoubleBufferedAnimatedWindowHandler::draw_one_move(&mut turtle,
                                                                       &mut self.iters[bufnum]);
                    DoubleBufferedAnimatedWindowHandler::draw_one_move(&mut turtle,
                                                                       &mut self.iters[bufnum]);
                }
            }
        }
    }
}