use player::Player;

pub struct WorldState {
    players: Vec<Player>
}

impl WorldState {
    pub fn new() -> WorldState {
        WorldState {
            players: Vec::new()
        }
    }

    pub fn mut_get_players(&mut self) -> &mut Vec<Player> {
        &mut self.players
    }

    pub fn get_players(&self) -> &Vec<Player> {
        &self.players
    }
}

