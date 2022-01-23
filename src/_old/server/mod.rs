mod child;

use crate::state::State;

pub enum Comms {
    State(State),
    Exit
}

pub trait Server {
    fn poll_comms(&mut self) -> Option<Comms>;
}
