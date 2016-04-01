use CompletionState::*;
use Direction::*;
use Event::*;

fn main() {
    let events = vec![Click, CycleCompleted, Click, Click, Click, Click, Click, CycleCompleted];

    let mut state = State {
        completion_state: Complete,
        direction: Closing,
    };

    for event in events {
        println!("Door: {:?} {:?}", state.completion_state, state.direction);
        println!("> {:?}", &event);
        state = state.transition(event);            
    }

    println!("Door: {:?} {:?}", state.completion_state, state.direction);
}

struct State {
    completion_state: CompletionState,
    direction: Direction,
}

impl State {
    fn new(completion_state: CompletionState, direction: Direction) -> State {
        State { completion_state: completion_state, direction: direction }
    }

    fn transition(&self, event: Event) -> State {
        match (self.completion_state.clone(), self.direction.clone(), event) {
            (InProgress, direction, Click         ) => State::new(Stopped   , direction),
            (InProgress, direction, CycleCompleted) => State::new(Complete  , direction),
            (_         , direction, Click         ) => State::new(InProgress, direction.flip()),
            _ => panic!("Invalid input"),
        }
    }
}

#[derive(Clone, Debug)]
enum CompletionState {
    Complete,
    InProgress,
    Stopped,
}

#[derive(Clone, Debug)]
enum Direction {
    Opening,
    Closing,
}

impl Direction {
    fn flip(&self) -> Direction {
        match *self {
            Opening => Closing,
            Closing => Opening,
        }
    }
}

#[derive(Debug)]
enum Event {
    Click,
    CycleCompleted,
}
