use std::collections::HashMap;

use player::Player;

pub type PlayerMap = HashMap<usize, Player>;

pub struct WorldState {
    players: PlayerMap
}

impl WorldState {
    pub fn new() -> WorldState {
        WorldState {
            players: PlayerMap::new()
        }
    }

    pub fn mut_get_players(&mut self) -> &mut PlayerMap {
        &mut self.players
    }

    pub fn get_players(&self) -> &PlayerMap {
        &self.players
    }
}

