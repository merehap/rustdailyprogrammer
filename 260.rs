
fn main() {
    let events = vec![
        Event::Click,
        Event::CycleCompleted,
        Event::Click,
        Event::Click,
        Event::Click,
        Event::Click,
        Event::Click,
        Event::CycleCompleted,
    ];

    let mut state = State {
        completion_state: CompletionState::Complete,
        direction: Direction::Closing,
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
            (CompletionState::InProgress, direction, Event::Click) =>
                State::new(CompletionState::Stopped, direction),
            (_, direction, Event::Click) =>
                State::new(CompletionState::InProgress, direction.flip()),
            (CompletionState::InProgress, direction, Event::CycleCompleted) =>
                State::new(CompletionState::Complete, direction),
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
            Direction::Opening => Direction::Closing,
            Direction::Closing => Direction::Opening,
        }
    }
}

#[derive(Debug)]
enum Event {
    Click,
    CycleCompleted,
}
