use crate::types::{Clue, Params, Round, Seed, ShipId};
use std::fmt::{Display, Formatter};

/// island's response to a clue request by a ship.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IslandResponse {
    /// pass an instance of the island's clue weakened or full.
    Respond(Clue),
    /// The island refused to give the clue to the requesting ship due to the island's policies.
    Refused,
    /// The requesting ship has to wait at least n rounds before receiving a version of the clue.
    Wait(u32),
    /// The requesting ship has already received the full strength clue.
    NothingMore,
}

/// island's response to a moor request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoorResponse {
    /// The island approved the ship's mooring request.
    Accepted,
    /// The island rejects the request due to its policies.
    Refused,
    /// The island rejects the request due to full berth.
    NoBerth,
}


/// island's response to a repair_hull request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairHullResponse {
    /// the island repairs the requesting ship's hull by 'h' hulls. To be discussed of whose responsibility is to check if old_hull + h > hull_max
    Accepted(u32),
    /// request rejected due to island own policies.
    Refused,
    /// (n > 0) The ships has to wait at least n rounds to be repaired.
    Wait(u32),
}


impl Display for IslandResponse{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            IslandResponse::Respond(clue) => write!(f,"Response with clue: {:?}", clue),
            IslandResponse::Wait(round) => write!(f,"Must wait {:?} rounds", round),
            IslandResponse::Refused => write!(f,"Refused"),
            IslandResponse::NothingMore => write!(f,"Nothing More"),
        }
    }
}



impl Display for MoorResponse{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            MoorResponse::Accepted => write!(f,"Accepted"),
            MoorResponse::NoBerth => write!(f,"NoSpace"),
            MoorResponse::Refused => write!(f,"Refused"),
        }
    }
}

// Understand how the chart creates the islands, should there be a factory function or new is good enough

/// **TBD**
#[derive(Debug, Clone, Copy)] //Struct in cui riceve da Chart tutte cose
pub struct IslandInit<'a> {
    pub clue: Clue,
    pub seed: Seed, // PRNG initialization since chart sends its randomness to each Island we guessed its gonna be something similar to wind function, so like an IslandSeed
    pub params: &'a Params, //send a copy not reference
}

// WG Stuff:
/*
can moor and moor, what was patrignani talkin bout

Chart island initialization, tbd

how repairHull behaves, tbd

askHistory method, tbd

how prng is passed from Chart to each island must ask rn

comment code and publish on common code?
*/

pub trait Island {

    fn new(init: IslandInit<'_>) -> Self; //Constructor, chart calls: Box::new(CertainIsland::new(init1));

    /// **applies** a valid moor request
    ///The chart calls this method to **apply** the moor request from Ship S to the Island I.
    ///
    /// This must be called **after** the chart validates the request and only when the request is Approved against geometry rules and island policies.
    ///
    /// moor method mutates the island's state
    fn moor(&mut self, ship: ShipId);

    /// validates a moor request
    /// this method is called by the Chart during the moor request's validation process.
    ///
    /// the method only checks the request against the island's policies and berth capacity; it does not check geometry constraints and other game rules.
    fn can_moor(&self, ship: ShipId) -> MoorResponse; // read-only method

    fn is_moored(&self, ship: ShipId) -> bool;

    /// makes a moored ship to leave the berth
    /// this method is called by the chart **after** validating that the ship is actually moored.
    fn leave_berth(&mut self, ship: ShipId);  // using a result for eventual  panics that shouldnt but might happen


    fn ask_clue(&mut self, ship: ShipId) -> IslandResponse;

    fn remaining_wait(&self, ship: ShipId) -> Option<u32>; // should just be u32 cuz it technically cannot be ever None

    fn has_more_to_offer(&self, ship: ShipId) -> bool;

    fn round_advance(&mut self, round: Round); //Chart handles the global rounds and communicates them to the island through this function

    //Optional stuff that might be good to add

    fn repair_hull(&self, ship: ShipId) -> RepairHullResponse;

    fn ask_history(&self, ship: ShipId) -> Option<Vec<VisitRecord>>;
}

//optional struct for storing records of visits for each Ship
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisitRecord {
    pub ship: ShipId,
    pub arrival_round: Round,
    pub departure_round: Option<Round>,
}
