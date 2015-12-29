// Copyright (c) 2015 William (B.J.) Snow Orvis
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

use common::{Point, Turtle, TurtleApp};

/// Represents a lindenmayer system. It requires an alphabet (represented as an enum), an initial
/// sequence ("string"), and one or more rules that transform the sequence with each
/// iteration/generation.
pub trait LindenmayerSystem<Alphabet: Clone> {
    /// Should return the initial Lindenmayer system string (iteration 0).
    fn initial() -> Vec<Alphabet>;

    /// Apply Lindenmayer system rules to a given character.
    ///
    /// This is done to efficiently use `match`ing at compile time, rather than returning a hashmap
    /// and handling it at runtime.
    fn apply_rule(Alphabet) -> Vec<Alphabet>;

    /// Generates a Lindenmayer system string for `iteration`.
    ///
    /// This is done by starting with the initial sequence, and repeatedly applying the rules to
    /// the sequence `iteration` times. The result is a new vector that contains the sequence for
    /// the specified iteration.
    fn generate(iteration: u64) -> Vec<Alphabet> {
        let mut lstr: Vec<Alphabet> = Self::initial();
        let mut i = 0;
        while i < iteration {
            i = i + 1;
            let mut newlstr: Vec<Alphabet> = vec![];

            for l in lstr.iter().cloned() {
                for other in Self::apply_rule(l).iter().cloned() {
                    newlstr.push(other);
                }
            }
            lstr = newlstr;
        }
        lstr
    }
}

/// In order to draw a fractal using a Lindenmayer System, we need to translate the output from the
/// L-System into turtle commands. To do this, we need to initial the turtle, and we need a way to
/// convert the L-System's symbols into actions. This trait provides the methods needed to
/// configure the LindenmayerSystemTurtleApp, which is the glue that issues arbitrary Turtle
/// commands.
pub trait LindenmayerSystemDrawingParameters<Alphabet> {
    /// Returns the iteration that should be drawn.
    fn iteration(&self) -> u64;

    /// Set up the turtle's initial position and direction.
    ///
    /// If no direction is specified, then the turtle will start at (0.0, 0.0). Most IFS fractals
    /// have some sort of formula for their initial angle that will ensure that the fractal is
    /// drawn within the viewing space.
    fn initialize_turtle(&self, turtle: &mut Turtle);

    /// Convert symbol into turtle commands.
    ///
    /// Usually, when moving the turtle forwards, there is some formula that will ensure that the
    /// turtle always ends at a given point, such as at (1.0, 0.0).
    fn interpret_symbol(&self, symbol: Alphabet, turtle: &mut Turtle);
}

use std::marker::PhantomData;

pub struct LindenmayerSystemTurtleApp<L, A>
    where L: LindenmayerSystem<A> + LindenmayerSystemDrawingParameters<A>,
          A: Clone
{
    alphabet: PhantomData<A>,
    system: L,
}

impl<L, A> LindenmayerSystemTurtleApp<L, A>
    where L: LindenmayerSystem<A> + LindenmayerSystemDrawingParameters<A>,
          A: Clone
{
    pub fn new(system: L) -> LindenmayerSystemTurtleApp<L, A> {
        LindenmayerSystemTurtleApp {
            system: system,
            alphabet: PhantomData,
        }
    }
}

impl<L, A> TurtleApp for LindenmayerSystemTurtleApp<L, A>
    where L: LindenmayerSystem<A> + LindenmayerSystemDrawingParameters<A>,
          A: Clone
{
    fn draw(&self, turtle: &mut Turtle) {
        println!("Generating L-System sequence...");
        let sequence = L::generate(self.system.iteration());
        println!("Done");

        turtle.set_pos(Point { x: 0.0, y: 0.0 });
        self.system.initialize_turtle(turtle);
        turtle.down();

        for symbol in sequence.iter().cloned() {
            self.system.interpret_symbol(symbol, turtle);
        }
        turtle.up();
    }
}


#[cfg(test)]
mod test {
    use super::LindenmayerSystem;

    #[derive(Clone, PartialEq, Eq, Debug)]
    enum TestABC {
        A,
        B,
        C,
        Foo,
    }

    struct TestLS;

    impl LindenmayerSystem<TestABC> for TestLS {
        fn initial() -> Vec<TestABC> {
            vec![TestABC::A, TestABC::B, TestABC::C]
        }

        fn apply_rule(l: TestABC) -> Vec<TestABC> {
            match l {
                TestABC::A => vec![TestABC::A, TestABC::Foo, TestABC::B, TestABC::C],
                TestABC::B => vec![TestABC::Foo],
                TestABC::C => vec![TestABC::Foo, TestABC::B],
                TestABC::Foo => vec![TestABC::Foo],
            }
        }
    }

    #[test]
    fn test_ls() {
        assert_eq!(TestLS::generate(0), [TestABC::A, TestABC::B, TestABC::C]);
        assert_eq!(TestLS::generate(1),
                   [TestABC::A,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::C,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::B]);
        assert_eq!(TestLS::generate(2),
                   [TestABC::A,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::C,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo]);
        assert_eq!(TestLS::generate(3),
                   [TestABC::A,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::C,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo]);
    }
}