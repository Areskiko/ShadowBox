use rand::Rng;

pub enum Target {
    TopLeft,
    TopRight,
    Bottom,
}
pub enum Action {
    Attack(Target),
    Block(Target),
}

pub struct Set {
    pauses: Vec<u16>,
    actions: Vec<Action>,
}

impl Set {
    pub fn new() -> Self {
        let mut pauses = Vec::new();
        let mut actions = Vec::new();
        let mut rng = rand::thread_rng();

        pauses.push(rng.gen_range(300, 1300));

        // Generate a random set of pauses and actions
        for _ in 0..rng.gen_range(1, 10) {
            let pause = rng.gen_range(200, 700);
            pauses.push(pause);
            let target = match rng.gen_range(0, 3) {
                0 => Target::TopLeft,
                1 => Target::TopRight,
                2 => Target::Bottom,
                _ => panic!("Invalid target"),
            };
            let action = match rng.gen_range(0, 2) {
                0 => Action::Attack(target),
                1 => Action::Attack(target),
                _ => Action::Attack(target),
            };
            actions.push(action);
        }

        Set { pauses, actions }
    }

    pub fn empty() -> Self {
        Set {
            pauses: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn add_pause(&mut self, pause: u16) {
        self.pauses.push(pause);
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn get_pauses(&self) -> &Vec<u16> {
        &self.pauses
    }

    pub fn get_actions(&self) -> &Vec<Action> {
        &self.actions
    }
}
