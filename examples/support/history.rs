//! Esempi di stato PRIVATO delle Island, non parte dell'API shared.
//! Altri gruppi possono scegliere strutture completamente differenti.

use std::collections::BTreeMap;

use IslandSharedType::types::{Round, ShipId};

#[derive(Debug, Default)]
pub struct IslandHistory {
    visits: BTreeMap<ShipId, Vec<Round>>,
}

impl IslandHistory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_visit(&mut self, ship: ShipId, round: Round) {
        self.visits.entry(ship).or_default().push(round);
    }

    pub fn visits_of(&self, ship: ShipId) -> Option<&[Round]> {
        self.visits.get(&ship).map(Vec::as_slice)
    }
}

/// Politica di attesa scelta ESCLUSIVAMENTE da GreedyIsland.
/// Non e' un contratto obbligatorio per tutte le implementazioni di Island.
#[derive(Debug, Clone, Copy)]
pub struct ClueContract {
    pub start_round: Round,
    pub required_rounds: u32,
}

impl ClueContract {
    pub fn remaining_at(&self, round: Round) -> u32 {
        let elapsed = round.saturating_sub(self.start_round);
        self.required_rounds.saturating_sub(elapsed)
    }
}
