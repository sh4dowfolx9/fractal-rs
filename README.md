# fractal-rs

A Rust application (and library) for exploring a variety of fractals and to
learn more about Rust.

Features include:

* Piston-based turtle graphics renderer for line-drawn curves
* Ability to animate the drawing of the curves
* Library support for computing turtle directions using the [Lindenmeyer
  system](https://en.wikipedia.org/wiki/L-system)
* Curves supported:
    * [Cesàro square fractal (torn
      fractal)](http://mathworld.wolfram.com/CesaroFractal.html)
    * Cesàro triangle fractal
    * [Dragon curve](https://en.wikipedia.org/wiki/Dragon_curve)
    * [Koch snowflake](https://en.wikipedia.org/wiki/Koch_snowflake)
    * [Lévy C curve](https://en.wikipedia.org/wiki/L%C3%A9vy_C_curve)
    * Terdragon fractal

Some future ideas (in no particular order):

* Option to automatically profile and adjust how much can be animated per-frame
  based on system performance.
* Display information about the current fractal.
* Greater interactivity, maybe a UI for choosing and configuring which fractal
  to display, or arrow keys to increment/decrement the iteration number.
* Color for various curves
* Ability to export images or animations
* Dynamically specify a curve's parameters through configuration instead of
  compiling them in. Alternately, make it easy to write a rust module for a
  curve and add it to the application loop.
* More [Iterated Function
  Systems](https://en.wikipedia.org/wiki/Iterated_function_system), such as
  those drawn using the chaos game, or that can be draw with shapes
* Other kinds of fractals like Mandelbrot sets, Julia sets, Burning ship
  fractal, etc.
* Explore using threading and channels to construct a generic iterator of turtle
  program steps (simulating coroutines). This might allow for more programming
  styles within a TurtleProgram instead of having to create custom iterators
  for each TurtleProgram implementation that have to track state/program
  counter for the program. It would also be a great exercise in multi-threading.


## Usage

Fetch the git repository, and then use cargo to build it:

```sh
git clone https://github.com/aetherknight/fractal-rs.git
cd fractal-rs
cargo build
```

To run the application, you can use `cargo run` or call the binary directly
(Eg, if you use `cargo install` to install the program). Resizing the window
will dynamically resize the curve as well. You can exit by pressing the `esc`
key.

To open a window and draw iteration 4 of the Cesàro square fractal:

```sh
cargo run -- cesaro 4
```

To animate the drawing of a curve, you can use the `--animate` option. You must
specify the number of line segments that should be drawn per frame as well,
such as `1` for one line per frame. The following will animate iteration 11 of
the dragon fractal at a rate of 10 line segments per frame:

```sh
cargo run -- --animate 10 dragon 11
```


### Exploring Fractals

TODO: Usage for all supported fractals

Note that for most fractals the iteration number results in an exponential
increase in computation, so if you want to explore a higher
iteration/generation of a curve, you may want to start with a low iteration
number and increment your way up. (At the moment, the dragon fractal tends to
take more than a few seconds to draw iterations above 15 my laptop when it
draws the entire curve in a single frame).

At present, the other parameters that make up one of the curves --- such as the
coordinate space, the L-system used to determine the drawing steps, or the
angles chosen --- require changing the source code.

## Contributing

* If you have a feature request and would like to discuss it, please open a
  ticket.
* If you would like to implement a feature, feel free to submit a pull request
  or open a ticket to discuss followed by a pull request.
* If I receive many contributions, I will shift the project's copyright
  structure to reference a contributers file.

A few rules about contributed code:

* In general, contributions should work on the current stable release of Rust.
  If you want to use a nightly Rust feature, we should discuss the approach
  (eg, can it be made optional, does it provide a huge performance improvement,
  etc.).
* Use [rustfmt](https://github.com/rust-lang-nursery/rustfmt) to format your
  code before committing.
* Write tests where it makes sense to do so (ie, test behaviors and
  functionality that could change as a side-effect of some other change), but
  do not fret about it.


## License

Copyright (c) 2015-2016 William (B.J.) Snow Orvis. Licensed under the Apache
License, Version 2.0. See [LICENSE](LICENSE) for details.
