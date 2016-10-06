mod gumball {
    pub trait State {
        fn insertQuarter(self) -> Box<State>;
        fn ejectQuarter(self) -> Box<State>;
/*
        fn turnCrunk(self) -> Box<State>;
        fn dispense(self) ->Box<State>;

*/
    }

    pub struct GumBallMachine {
        state : Box<State>
    }
    impl GumBallMachine {
        pub fn new() -> Self {
            GumBallMachine { state : Box::new(WaitQuarter{count : 0})}
        }
        pub fn loadBalls(&mut self, count: u32) {

        }
    }

    struct WaitQuarter {
        count : usize
    }
    impl State for WaitQuarter {
        fn insertQuarter(self) -> Box<State> {
            println!("A Quarter was inserted");
            Box::new(HasQuarter { count : self.count })
        }
        fn ejectQuarter(self) -> Box<State> {
            println!("Can't eject a Quarter. Has no Quarter");
            Box::new(self)
        }
    }

    struct HasQuarter {
        count : usize
    }
    impl State for HasQuarter {
        fn insertQuarter(self) -> Box<State> {
            println!("Can't insert a Quarter. Already has a Quarter");
            Box::new(self)
        }
        fn ejectQuarter(self) -> Box<State> {
            println!("Has returned Quarter");
            Box::new(WaitQuarter {count : self.count})
        }

    }

    struct SoldGum {
        count : usize
    }
    impl State for SoldGum {
        fn insertQuarter(self) -> Box<State> {
            println!("Already has a Quarter");
            Box::new(self)
        }
        fn ejectQuarter(self) -> Box<State> {
            println!("Can't insert a Quarter. The GumBall was sold");
            Box::new(self)
        }
    }

    struct StockOut {
        count : usize
    }
    impl State for StockOut {
        fn insertQuarter(self) -> Box<State> {
            println!("Out of stock");
            Box::new(self)
        }
        fn ejectQuarter(self) -> Box<State> {
            println!("Can't eject a Quarter. Has no Quarter");
            Box::new(self)
        }

    }

}

use gumball::GumBallMachine;

fn main() {
    println!("Hello, world!");
    let machine = GumBallMachine::new();
}
