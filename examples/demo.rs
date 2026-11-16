//! Esempi di due implementazioni diverse dello STESSO trait condiviso.
//!
//! Eseguire con: cargo run --example demo
//!
//! Questo file e' un mock dell'interfaccia Chart-Island, NON un motore di gioco.
//! Per semplicita' utilizza soltanto Clue { id } e non implementa il weakening.
//! I round partono da 1, come indicato dal Protocol.
//!
//! Le strutture in support/history.rs sono private agli esempi e NON sono shared types.

#[path = "support/history.rs"]
mod history;

use std::collections::{BTreeMap, BTreeSet};

use IslandSharedType::island::{Island, IslandContext, IslandResponse, MoorResponse};
use IslandSharedType::types::{Clue, Params, ShipId};

use history::{ClueContract, IslandHistory};

// Island semplice: Clue completo immediato dopo l'ormeggio.
struct GenerousIsland {
    clue: Clue,
    moored_ships: BTreeSet<ShipId>,
    served_ships: BTreeSet<ShipId>,
    history: IslandHistory,
}

impl GenerousIsland {
    fn new(clue: Clue) -> Self {
        Self {
            clue,
            moored_ships: BTreeSet::new(),
            served_ships: BTreeSet::new(),
            history: IslandHistory::new(),
        }
    }
}

impl Island for GenerousIsland {
    fn moor(&mut self, ship: ShipId, ctx: &IslandContext<'_>) -> MoorResponse {
        if self.moored_ships.contains(&ship) || !self.has_more_to_offer(ship) {
            return MoorResponse::Refused;
        }

        self.moored_ships.insert(ship);
        self.history.record_visit(ship, ctx.round);
        MoorResponse::Accepted
    }

    fn leave_berth(&mut self, ship: ShipId, _ctx: &IslandContext<'_>) {
        self.moored_ships.remove(&ship);
    }

    fn ask_clue(&mut self, ship: ShipId, _ctx: &IslandContext<'_>) -> IslandResponse {
        if !self.moored_ships.contains(&ship) {
            return IslandResponse::Refused;
        }
        if !self.has_more_to_offer(ship) {
            return IslandResponse::NothingMore;
        }

        self.served_ships.insert(ship);
        IslandResponse::Respond(self.clue.clone())
    }

    fn remaining_wait(&self, ship: ShipId, _ctx: &IslandContext<'_>) -> Option<u32> {
        if self.moored_ships.contains(&ship) && self.has_more_to_offer(ship) {
            Some(0)
        } else {
            None
        }
    }

    fn has_more_to_offer(&self, ship: ShipId) -> bool {
        !self.served_ships.contains(&ship)
    }

    fn on_round_advance(&mut self, ctx: &IslandContext) {
        ctx.round = ctx.round +1;
    }
}

// Island che applica un'attesa fissa alla singola visita.
// Questa e' una strategia di esempio: il trait NON impone ClueContract.
struct GreedyIsland {
    clue: Clue,
    contracts: BTreeMap<ShipId, ClueContract>,
    served_ships: BTreeSet<ShipId>,
    history: IslandHistory,
}

impl GreedyIsland {
    fn new(clue: Clue) -> Self {
        Self {
            clue,
            contracts: BTreeMap::new(),
            served_ships: BTreeSet::new(),
            history: IslandHistory::new(),
        }
    }

    fn price_for(&self, ship: ShipId) -> u32 {
        let base: u32 = match ship {
            0 => 3,
            1 => 7,
            _ => 5,
        };
        // Esempio di politica interna: sconto per visite precedenti.
        let already_visited = self.history.visits_of(ship).is_some();
        if already_visited { base.saturating_sub(1) } else { base }
    }
}

impl Island for GreedyIsland {
    fn moor(&mut self, ship: ShipId, ctx: &IslandContext<'_>) -> MoorResponse {
        if self.contracts.contains_key(&ship) || !self.has_more_to_offer(ship) {
            return MoorResponse::Refused;
        }
        // Capacita' di 2 berth: solo una scelta di questa implementazione.
        if self.contracts.len() >= 2 {
            return MoorResponse::NoSpace;
        }

        let contract = ClueContract {
            start_round: ctx.round,
            required_rounds: self.price_for(ship),
        };
        self.contracts.insert(ship, contract);
        self.history.record_visit(ship, ctx.round);
        MoorResponse::Accepted
    }

    fn leave_berth(&mut self, ship: ShipId, _ctx: &IslandContext<'_>) {
        // In questo esempio lasciare il berth azzera l'attesa attiva.
        self.contracts.remove(&ship);
    }

    fn ask_clue(&mut self, ship: ShipId, ctx: &IslandContext<'_>) -> IslandResponse {
        if !self.contracts.contains_key(&ship) {
            return IslandResponse::Refused;
        }
        if !self.has_more_to_offer(ship) {
            return IslandResponse::NothingMore;
        }

        let Some(remaining) = self.remaining_wait(ship, ctx) else {
            return IslandResponse::Refused;
        };
        if remaining > 0 {
            return IslandResponse::Wait(remaining);
        }

        self.served_ships.insert(ship);
        IslandResponse::Respond(self.clue.clone())
    }

    fn remaining_wait(&self, ship: ShipId, ctx: &IslandContext<'_>) -> Option<u32> {
        if !self.has_more_to_offer(ship) {
            return None;
        }
        self.contracts.get(&ship).map(|contract| contract.remaining_at(ctx.round))
    }

    fn has_more_to_offer(&self, ship: ShipId) -> bool {
        !self.served_ships.contains(&ship)
    }
}

// Le due demo sono scenari SEPARATI, non due Island visitate dalla
// stessa Ship nello stesso round. Ogni Ship esegue al massimo un'azione
// per round; remaining_wait() simula una query interna della Chart.
fn demo_generous(island: &mut dyn Island, params: &Params) {
    println!("\n=== GENEROUS ISLAND ===");

    let ctx = IslandContext { round: 1, params };
    println!("Round 1, Ship 0: Moor -> {}", island.moor(0, &ctx));
    assert_eq!(island.remaining_wait(0, &ctx), Some(0));

    let ctx = IslandContext { round: 2, params };
    println!("Round 2, Ship 0: AskClue -> {}", island.ask_clue(0, &ctx));
    assert!(!island.has_more_to_offer(0));

    let ctx = IslandContext { round: 3, params };
    println!("Round 3, Ship 0: AskClue -> {}", island.ask_clue(0, &ctx));

    // La Chart notifica l'uscita quando la Ship esegue Move.
    let ctx = IslandContext { round: 4, params };
    island.leave_berth(0, &ctx);
    println!("Round 4, Ship 0: Move -> berth released");
}

fn demo_greedy(island: &mut dyn Island, params: &Params) {
    println!("\n=== GREEDY ISLAND ===");

    let ctx = IslandContext { round: 1, params };
    println!("Round 1, Ship 0: Moor -> {}", island.moor(0, &ctx));
    println!("Round 1, Ship 1: Moor -> {}", island.moor(1, &ctx));
    println!("Round 1, Ship 2: Moor -> {}", island.moor(2, &ctx)); // NoSpace

    let ctx = IslandContext { round: 2, params };
    println!("Round 2, Ship 0: AskClue -> {}", island.ask_clue(0, &ctx));
    println!("Round 2, Ship 1: AskClue -> {}", island.ask_clue(1, &ctx));

    let ctx = IslandContext { round: 3, params };
    println!("Round 3 (Chart query), Ship 0 remaining: {:?}", island.remaining_wait(0, &ctx));
    // Nel round 3 le Ship possono scegliere Hold: non chiamiamo ask_clue.

    let ctx = IslandContext { round: 4, params };
    println!("Round 4, Ship 0: AskClue -> {}", island.ask_clue(0, &ctx));
    println!("Round 4 (Chart query), Ship 1 remaining: {:?}", island.remaining_wait(1, &ctx));

    let ctx = IslandContext { round: 8, params };
    println!("Round 8, Ship 1: AskClue -> {}", island.ask_clue(1, &ctx));

    let ctx = IslandContext { round: 9, params };
    println!("Round 9, Ship 0: AskClue -> {}", island.ask_clue(0, &ctx));
    println!("Round 9, Ship 1: AskClue -> {}", island.ask_clue(1, &ctx));

    assert!(!island.has_more_to_offer(0));
    assert!(!island.has_more_to_offer(1));
}

fn main() {
    // Params e' #[non_exhaustive]: dall'esempio non si puo' costruire con un literal.
    // E' necessario il Default implementato nel crate che definisce Params.
    let params = Params::default();
    println!("Game round cap: {}", params.round_cap);

    let clue = Clue { id: 42 };
    let mut islands: Vec<Box<dyn Island>> = vec![
        Box::new(GenerousIsland::new(clue.clone())),
        Box::new(GreedyIsland::new(clue)),
    ];

    demo_generous(islands[0].as_mut(), &params);
    demo_greedy(islands[1].as_mut(), &params);
}
