mod gumball {

    #[derive(PartialEq, Eq)]
    enum State {
        WaitQuarter,
        HasQuarter,
        SoldGum,
        OutOfStock,
    }

    pub struct GumBallMachine {
        count : usize,
        state : State
    }
    impl GumBallMachine {
        pub fn new() -> Self {
            GumBallMachine {
                count : 0,
                state : State::OutOfStock
            }
        }

        pub fn load(mut self, count: usize) ->Self {
            println!("Loaded {} Gum Balls", &count);
            self.count += count;
            if self.state == State::OutOfStock {
                self.state = State::WaitQuarter;
            }
            self
        }

        pub fn insert(mut self) -> Self{
            if self.state == State::WaitQuarter {
                println!("Quarter was accepted");
                self.state = State::HasQuarter;
            } else {
                println!("Quarter was NOT accepted");
            }
            self
        }

        pub fn eject(mut self) -> Self{
            if self.state == State::HasQuarter {
                println!("Quarter was EJECTED");
                self.state = State::WaitQuarter;
            } else {
                println!("No Quarter to eject");
            }
            self
        }

        pub fn buy(mut self) -> Self{
             match self.state {
                State::OutOfStock => {
                    println!("Out of Stock");
                },
                State::WaitQuarter => {
                    println!("Waiting for Quarter");
                },
                State::HasQuarter => {
                    println!("Gum was sold");
                    self.state = State::SoldGum;
                    self.dispense();
                },
                State::SoldGum => {
                    println!("Gum was already sold");
                },
            };
            self
        }

        fn dispense(&mut self) {
            self.count -= 1;
            println!("Gum was rolled out to the slot");
            if self.count == 0  {
                self.state = State::OutOfStock
            } else {
                self.state = State::WaitQuarter
            }
        }
    }
}

use std::rc::Rc;
use gumball::GumBallMachine;


fn main() {
    println!("BEGIN!!!!");
    GumBallMachine::new().load(4).insert().buy();
    GumBallMachine::new().load(4).insert().eject().buy();
    println!("END!!!!");

}
