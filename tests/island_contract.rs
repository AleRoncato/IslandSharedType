use std::collections::HashSet;
use IslandSharedType::island::{
    Island, IslandInit, IslandResponse,
    MoorResponse, RepairHullResponse,
};
use IslandSharedType::types::{Clue, Round, ShipId};
// creazione di una isola di prova
struct MockIsland {
    clue: Clue,
    moored: Option<ShipId>,
    served_full: HashSet<ShipId>,
    remaining: u32,
}

impl Island for MockIsland {
    // metodo new per creare una isola
    fn new(init: IslandInit) -> Self {
        // crea una isola con un solo posto di attracco
        // quando creiamo l'isola, nessuna nave è ancora ormeggiata
        // terzo punto per il quarto test creiamo una served
        Self {
            clue: init.clue,
            moored: None,
            served_full: HashSet::new(),
            remaining: 3,
        }
    }
    // registra effettivamente l'ormeggio
    fn moor(&mut self, ship: ShipId) {
        self.moored = Some(ship);
    }
    // controlla se una nave può ormeggiare, senza cambiare lo stato dell'isola

    fn can_moor(&self, _ship: ShipId) -> MoorResponse {
        if self.moored.is_none() {
            MoorResponse::Accepted
        } else {
            MoorResponse::NoBerth
        }
    }
    //verifica se una nave è ormeggiata
    fn is_moored(&self, ship: ShipId) -> bool {
        self.moored == Some(ship)
    }
    //registra l'uscita della nave
    fn leave_berth(&mut self, ship: ShipId) {
        if self.moored == Some(ship) {
            self.moored = None;
        }
    }

    fn ask_clue(&mut self, ship: ShipId) -> IslandResponse {
        if self.served_full.contains(&ship){
            return IslandResponse::NothingMore
        }
        if self.remaining>0{
            return IslandResponse::Wait(self.remaining);
        }
        self.served_full.insert(ship);
        IslandResponse::Respond(self.clue)

        // IslandResponse::Respond(self.clue)


    }

    fn remaining_wait(&self, ship: ShipId) -> Option<u32> {
        Some(self.remaining)
    }

    fn has_more_to_offer(&self, ship: ShipId) -> bool {
        if self.served_full.contains(&ship){
            return false
        }
        true
    }

    fn round_advance(&mut self, _round: Round) {

        if self.remaining>0{
            self.remaining-=1;
        }// niente else?

    }


    fn repair_hull(&self, _ship: ShipId) -> RepairHullResponse {
        RepairHullResponse::Refused
    }
}

#[test]
fn test_island_trait_object() {
    let init = IslandInit {
        clue: Clue { id: 1 },
        seed: 42,
        params: Default::default(),
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
        params: Default::default(),
    };

    let mock = MockIsland::new(init);

    assert_eq!(mock.ask_history(1), None);
}

#[test]
fn test_moor_lifecycle() {
    let init = IslandInit {
        clue: Clue { id: 1 },
        seed: 42,
        params: Default::default(),
    };

    let mut island = MockIsland::new(init);

    // All'inizio la nave 1 non è ormeggiata
    assert!(!island.is_moored(1));

    // Controlliamo se può ormeggiare
    assert_eq!(island.can_moor(1), MoorResponse::Accepted);

    // can_moor non deve modificare lo stato
    assert!(!island.is_moored(1));

    // Ormeggiamo effettivamente la nave
    island.moor(1);
    assert!(island.is_moored(1));

    // La nave 2 trova il posto occupato
    assert_eq!(island.can_moor(2), MoorResponse::NoBerth);

    // La nave 1 lascia l'isola
    island.leave_berth(1);
    assert!(!island.is_moored(1));

    // Il posto torna disponibile
    assert_eq!(island.can_moor(2), MoorResponse::Accepted);
}
#[test]
fn test_wait_for_clue() {
    let init = IslandInit {
        clue: Clue { id: 1 },
        seed: 42,
        params: Default::default(),
    };

    let mut island = MockIsland::new(init);
    island.moor(1);

    // Inizialmente mancano 3 round
    assert_eq!(island.ask_clue(1), IslandResponse::Wait(3));

    // Il Chart fa avanzare un round
    island.round_advance(1);

    // Ora ne mancano 2
    assert_eq!(island.ask_clue(1), IslandResponse::Wait(2));

    island.round_advance(2);
    assert_eq!(island.ask_clue(1), IslandResponse::Wait(1));

    island.round_advance(3);

    // L'attesa è terminata
    assert_eq!(island.remaining_wait(1), Some(0));

    // La nave riceve l'indizio completo
    assert_eq!(
        island.ask_clue(1),
        IslandResponse::Respond(Clue { id: 1 })
    );

    // Dopo aver ricevuto l'indizio completo,
    // l'isola non ha più nulla da offrirle
    assert!(!island.has_more_to_offer(1));

    // Una seconda richiesta non restituisce un altro indizio
    assert_eq!(
        island.ask_clue(1),
        IslandResponse::NothingMore
    );





}

