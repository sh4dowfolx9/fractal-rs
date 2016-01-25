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

extern crate argparse;
extern crate graphics;
extern crate piston;
extern crate piston_window;

// must be before any local modules that use the macros
#[macro_use]
mod macros;

pub mod curves;
pub mod geometry;
pub mod lindenmayer;
pub mod turtle;
mod glwindow;

use argparse::{ArgumentParser, Store, StoreTrue};

use curves::cesaro::CesaroFractal;
use curves::cesarotri::CesaroTriFractal;
use curves::dragon::DragonFractal;
use curves::kochcurve::KochCurve;
use curves::levyccurve::LevyCCurve;
use curves::terdragon::TerdragonFractal;
use lindenmayer::LindenmayerSystemTurtleProgram;
use turtle::TurtleProgram;

struct Arguments {
    curve_name: String,
    iterations: u64,
    animate: bool,
    version: bool,
}

fn parse_args() -> Arguments {
    let mut retargs = Arguments { curve_name: String::from(""), iterations: 0, animate: false, version: false };
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Renders fractal curves.");
        parser.refer(&mut retargs.animate).add_option(&["--animate"], StoreTrue, "Animate the drawing of the fractal instead of drawing it all at once.");
        parser.refer(&mut retargs.version).add_option(&["-v", "--version"], StoreTrue, "Display the version");

        parser.refer(&mut retargs.curve_name).add_argument("curve", Store, "Which curve to draw. Valid options are: cesaro, cesarotri, dragon, kochcurve, levyccurve, and terdragon.");
        parser.refer(&mut retargs.iterations).add_argument("iterations", Store, "The iteration of the specified curve to draw. should be a non-negative integer.");
        parser.parse_args_or_exit();
    }

    retargs
}

fn construct_program(program_name: &str, iterations: u64) -> Box<TurtleProgram> {
    match program_name {
        "cesaro"     => Box::new(LindenmayerSystemTurtleProgram::new(CesaroFractal::new(iterations).unwrap())),
        "cesarotri"  => Box::new(LindenmayerSystemTurtleProgram::new(CesaroTriFractal::new(iterations).unwrap())),
        "dragon"     => Box::new(DragonFractal::new(iterations).unwrap()),
        "kochcurve"  => Box::new(LindenmayerSystemTurtleProgram::new(KochCurve::new(iterations).unwrap())),
        "levyccurve" => Box::new(LindenmayerSystemTurtleProgram::new(LevyCCurve::new(iterations).unwrap())),
        "terdragon"  => Box::new(LindenmayerSystemTurtleProgram::new(TerdragonFractal::new(iterations).unwrap())),
        _            => panic!("Unknown program type")
    }
}

fn main() {
    let args = parse_args();

    let program = construct_program(args.curve_name.as_ref(), args.iterations);

    glwindow::run(&*program, args.animate);
}
