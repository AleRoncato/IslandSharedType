use island_trait::island::{Island, IslandInit, IslandResponse, MoorResponse};
use island_trait::types::{Clue, Params, Round, ShipId};
use std::collections::HashSet;
// Test Island with a single berth and a three-round wait per visit.
struct MockIsland {
    clue: Clue,
    moored: Option<ShipId>,
    served_full: HashSet<ShipId>,
    remaining: u32,
}

impl Island for MockIsland {
    fn new(init: IslandInit) -> Self {
        Self {
            clue: init.clue,
            moored: None,
            served_full: HashSet::new(),
            remaining: 0,
        }
    }
    fn moor(&mut self, ship: ShipId) {
        self.moored = Some(ship);
        self.remaining = 3;
    }

    fn can_moor(&self, _ship: ShipId) -> MoorResponse {
        if self.moored.is_none() {
            MoorResponse::Accepted
        } else {
            MoorResponse::NoBerth
        }
    }
    fn is_moored(&self, ship: ShipId) -> bool {
        self.moored == Some(ship)
    }
    fn leave_berth(&mut self, ship: ShipId) {
        if self.moored == Some(ship) {
            self.moored = None;
        }
    }

    fn ask_clue(&mut self, ship: ShipId) -> IslandResponse {
        if self.served_full.contains(&ship) {
            return IslandResponse::NothingMore;
        }
        if self.remaining > 0 {
            return IslandResponse::Wait(self.remaining);
        }
        self.served_full.insert(ship);
        IslandResponse::Respond(self.clue)
    }

    fn remaining_wait(&self, ship: ShipId) -> Option<u32> {
        (self.moored == Some(ship)).then_some(self.remaining)
    }

    fn has_more_to_offer(&self, ship: ShipId) -> bool {
        if self.served_full.contains(&ship) {
            return false;
        }
        true
    }

    fn round_advance(&mut self, _round: Round) {
        if self.moored.is_some() {
            self.remaining = self.remaining.saturating_sub(1);
        }
    }
}

#[test]
fn test_island_trait_object() {
    let init = IslandInit {
        clue: Clue { id: 1 },
        seed: 42,
        params: Params::default(),
    };

    let mut mock = MockIsland::new(init);

    let island: &mut dyn Island = &mut mock;

    assert_eq!(island.can_moor(1), MoorResponse::Accepted);
}

#[test]
fn test_default_ask_history() {
    let init = IslandInit {
        clue: Clue { id: 1 },
        seed: 42,
        params: Params::default(),
    };

    let mock = MockIsland::new(init);

    assert_eq!(mock.ask_history(1), None);
}

#[test]
fn test_moor_lifecycle() {
    let init = IslandInit {
        clue: Clue { id: 1 },
        seed: 42,
        params: Params::default(),
    };

    let mut island = MockIsland::new(init);

    assert!(!island.is_moored(1));

    assert_eq!(island.can_moor(1), MoorResponse::Accepted);

    // Checking availability leaves the berth unoccupied.
    assert!(!island.is_moored(1));

    island.moor(1);
    assert!(island.is_moored(1));

    assert_eq!(island.can_moor(2), MoorResponse::NoBerth);

    island.leave_berth(1);
    assert!(!island.is_moored(1));

    assert_eq!(island.can_moor(2), MoorResponse::Accepted);
}
#[test]
fn test_wait_for_clue() {
    let init = IslandInit {
        clue: Clue { id: 1 },
        seed: 42,
        params: Params::default(),
    };

    let mut island = MockIsland::new(init);
    island.moor(1);

    assert_eq!(island.ask_clue(1), IslandResponse::Wait(3));

    island.round_advance(1);

    assert_eq!(island.ask_clue(1), IslandResponse::Wait(2));

    island.round_advance(2);
    assert_eq!(island.ask_clue(1), IslandResponse::Wait(1));

    island.round_advance(3);

    assert_eq!(island.remaining_wait(1), Some(0));

    assert_eq!(island.ask_clue(1), IslandResponse::Respond(Clue { id: 1 }));

    // Delivering the full clue exhausts the offer for this ship.
    assert!(!island.has_more_to_offer(1));

    assert_eq!(island.ask_clue(1), IslandResponse::NothingMore);
}
#[test]
fn test_round_advance_zero() {
    let init = IslandInit {
        clue: Clue { id: 1 },
        seed: 42,
        params: Params::default(),
    };

    let mut island = MockIsland::new(init);

    island.moor(1);

    assert_eq!(island.remaining_wait(1), Some(3));

    island.round_advance(1);
    assert_eq!(island.remaining_wait(1), Some(2));

    island.round_advance(2);
    assert_eq!(island.remaining_wait(1), Some(1));

    island.round_advance(3);
    assert_eq!(island.remaining_wait(1), Some(0));

    island.round_advance(4);
    assert_eq!(island.remaining_wait(1), Some(0));

    island.round_advance(5);
    assert_eq!(island.remaining_wait(1), Some(0));
}

#[test]
fn test_multiple_ships() {
    let init = IslandInit {
        clue: Clue { id: 1 },
        seed: 42,
        params: Params::default(),
    };

    let mut island = MockIsland::new(init);
    island.moor(1);

    assert_eq!(island.ask_clue(1), IslandResponse::Wait(3));

    island.round_advance(1);

    assert_eq!(island.ask_clue(1), IslandResponse::Wait(2));

    island.round_advance(2);
    assert_eq!(island.ask_clue(1), IslandResponse::Wait(1));

    island.round_advance(3);

    assert_eq!(island.remaining_wait(1), Some(0));

    assert_eq!(island.ask_clue(1), IslandResponse::Respond(Clue { id: 1 }));
    island.leave_berth(1);

    island.moor(2);

    // The second ship starts its own three-round wait.
    assert_eq!(island.remaining_wait(2), Some(3));

    for round in 4..=6 {
        island.round_advance(round);
    }

    assert_eq!(island.remaining_wait(2), Some(0));

    assert_eq!(island.ask_clue(2), IslandResponse::Respond(Clue { id: 1 }));
}
#[test]
fn test_can_moor_is_pure() {
    let init = IslandInit {
        clue: Clue { id: 1 },
        seed: 42,
        params: Params::default(),
    };

    let mut island = MockIsland::new(init);
    assert_eq!(island.can_moor(1), MoorResponse::Accepted);
    assert!(!island.is_moored(1));
    island.moor(1);
    assert!(island.is_moored(1));
}
#[test]
fn test_leave_berth() {
    let init = IslandInit {
        clue: Clue { id: 1 },
        seed: 42,
        params: Params::default(),
    };

    let mut island = MockIsland::new(init);
    island.moor(1);
    assert!(island.is_moored(1));
    island.leave_berth(1);
    assert!(!island.is_moored(1));

    assert_eq!(island.can_moor(2), MoorResponse::Accepted);
    island.moor(2);
    assert!(island.is_moored(2));
}

#[test]
fn test_wait_starts_on_mooring() {
    let mut island = MockIsland::new(IslandInit {
        clue: Clue { id: 1 },
        seed: 42,
        params: Params::default(),
    });

    for round in 2..=4 {
        island.round_advance(round);
    }

    assert_eq!(island.can_moor(1), MoorResponse::Accepted);
    island.moor(1);

    assert_eq!(island.remaining_wait(1), Some(3));
    assert_eq!(island.ask_clue(1), IslandResponse::Wait(3));
}
