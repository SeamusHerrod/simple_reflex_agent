fn main() {
    
    //let agent1: Agent = Agent::new();
    //let name: i32 = 1;
    //print_perf(name, agent1);
    let mut name = 0;
    for config in &ALL_CONFIGS {
        let mut env = Environment{env_state: *config};
        let mut agent: Agent = Agent::new();
        agent.percept(&mut env);
        print_perf(name, agent);
        name += 1;
    }
}
#[derive(Clone, Copy)]
struct Environment {
    env_state: [(Location, Cleaned); 2],
}


#[derive(Debug)]
struct Agent {
    performance: i32,
}

impl Agent {
    fn new() -> Self {
        Self { performance: 0 }
    }

    fn percept(&mut self, env: &mut Environment) {
        // iterate through each cell in env
        for i in 0..2 {
            let (loc, clean) = env.env_state[i];
            if clean == Cleaned::DIRTY {
                self.performance += 1;
                env.env_state[i] = (loc, Cleaned::CLEAN);
            }
        }
    }
}
#[derive(Clone, Copy, PartialEq)]
enum Location {
    A,
    B,
    DEFAULT,
}
#[derive(Clone, Copy, PartialEq)]
enum Cleaned {
    CLEAN,
    DIRTY,
}

static ALL_CONFIGS: [[(Location, Cleaned); 2]; 8] = [
    [(Location::A, Cleaned::CLEAN), (Location::B, Cleaned::CLEAN)],
    [(Location::A, Cleaned::CLEAN), (Location::B, Cleaned::DIRTY)],
    [(Location::A, Cleaned::DIRTY), (Location::B, Cleaned::CLEAN)],
    [(Location::A, Cleaned::DIRTY), (Location::B, Cleaned::DIRTY)],
    [(Location::B, Cleaned::CLEAN), (Location::A, Cleaned::CLEAN)],
    [(Location::B, Cleaned::CLEAN), (Location::A, Cleaned::DIRTY)],
    [(Location::B, Cleaned::DIRTY), (Location::A, Cleaned::CLEAN)],
    [(Location::B, Cleaned::DIRTY), (Location::A, Cleaned::DIRTY)],
];

fn print_perf(name: i32, agent: Agent) {
    println!("Agent: {} Performance: {}", name, agent.performance);
}
