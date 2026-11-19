use std::collections::HashSet;
use IslandSharedType::island::{
    Island, IslandInit, IslandResponse,
    MoorResponse, RepairHullResponse,
};
use IslandSharedType::types::{Clue, Round, ShipId};
// creazione di una isola di prova
struct MockIsland {
    clue: Clue,
    moored: Option<ShipId>,// per salvare informazioni sulla barca
    // served_full: HashSet<ShipId>,// per aggiungere le barche cha hanno preso la full clue
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
            // served_full: HashSet::new(),
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
        // if self.served_full(ship){
        //     IslandResponse::Refused
        // }else{
        //     IslandResponse::Respond(self.clue)
        // }

        IslandResponse::Respond(self.clue)
    }

    fn remaining_wait(&self, _ship: ShipId) -> Option<u32> {
        None
    }
    fn has_more_to_offer(&self, _ship: ShipId) -> bool {
        true
    }

    fn round_advance(&mut self, _round: Round) {}

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
// #[test]
// fn test_ask_clue(){
//
// }

