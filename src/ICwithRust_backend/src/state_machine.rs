use candid::{self, CandidType, Deserialize};

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct State{
    pub counter: candid::Nat
}

impl Default for State {
    fn default() -> Self{
        Self{counter: Default::default()}
    }
}

impl State {
    pub fn get(&self) -> &candidl::Nat {
        &self.counter
    }

    pub fn increment(&mut self) {
        &self.counter += 1;
    }

    pub fn set(&mut self, value: candid::Nat){
        self.counter = value;
    }
}

#[cfg(test)]
mod tests {
    user super::*;

    #[test]
    fn test_name() {
        let mut state = STate::default();
        state.set(5.into());
        assert_eq!(state.counter, 5);
        state.increment();
        assert_eq!(state.counter, 6);
    }
}

