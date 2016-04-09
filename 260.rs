use CompletionState::*;
use Direction::*;
use Event::*;

fn main() {
    let events = vec![
        Click,
        CycleCompleted,
        Click,
        BlockDetected,
        Click,
        CycleCompleted,
        Click,
        BlockCleared,
        Click,
        CycleCompleted,
    ];

    let mut state = State {
        completion_state: Complete,
        direction: Closing,
        blocked: false,
    };

    for event in events {
        println!("Door: {:?} {:?}", state.completion_state, state.direction);
        println!("> {:?}", &event);
        state = state.transition(event);            
    }

    println!("Door: {:?} {:?}", state.completion_state, state.direction);
}

#[derive(Debug)]
struct State {
    completion_state: CompletionState,
    direction: Direction,
    blocked: bool,
}

impl State {
    fn new(completion_state: CompletionState, direction: Direction, blocked: bool) -> State {
        State { completion_state: completion_state, direction: direction, blocked: blocked }
    }

    fn transition(&self, event: Event) -> State {
        match (self.completion_state.clone(), self.direction.clone(), self.blocked, event.clone()) {
            (completion, direction, _      , BlockCleared  ) => State::new(completion, direction       , false  ),
            (InProgress, Closing  , _      , BlockDetected ) => State::new(InProgress, Opening         , true   ),
            (InProgress, direction, blocked, CycleCompleted) => State::new(Complete  , direction       , blocked),
            (completion, direction, true   , _             ) => State::new(completion, direction       , true   ),
            (InProgress, direction, blocked, Click         ) => State::new(Stopped   , direction       , blocked),
            (_         , direction, blocked, Click         ) => State::new(InProgress, direction.flip(), blocked),
            _ => panic!(format!("Invalid input. Event: {:?} . State: {:?} ", event, self)),
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

#[derive(Clone, Debug)]
enum Event {
    Click,
    CycleCompleted,
    BlockDetected,
    BlockCleared,
}
