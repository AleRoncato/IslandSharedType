use crate::types::{Clue, Params, Round, Seed, ShipId};
use std::fmt::{Display, Formatter};

/// How an island answers a request for its Clue.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IslandResponse {
    /// The Island gives the Clue weakened or full.
    Respond(Clue),
    /// The Island declines to answer this Ship, according to its own policy.
    Refused,
    /// The Ship must wait at least `n` more rounds before the Island answers.
    Wait(u32),
    /// The requesting Ship has already received the full-strength Clue.
    NothingMore,
}

/// How an Island answers to a moor request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoorResponse {
    /// The Island approves the Ship's mooring request.
    Accepted,
    /// The Island rejects the request according to its policies.
    Refused,
    /// The Island rejects the request due to full berth.
    NoBerth,
}


/// How an Island answer to a repair_hull request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairHullResponse {
    /// the Island repairs `ship` by `h` hulls.
    ///
    /// To be discussed of whose responsibility is to check if old_hull + h > hull_max
    Accepted(u32),
    /// The Island rejects the request according to its policies.
    Refused,
    /// The Ship has to wait at least `(n > 0)` rounds to be repaired.
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

    /// **Applies** a valid moor request
    ///The chart calls this method to **apply** the moor request from `ship`
    ///
    /// This must be called **after** the chart validates the request and only when the request
    /// is Accepted against geometry rules and island policies.
    ///
    /// moor method mutates the island's state by remembering `ship` being moored.
    fn moor(&mut self, ship: ShipId);

    /// Records that `ship` has moored here.
    ///
    /// Called by the Chart when applying a `Moor` action, immediately after
    /// [`Island::can_moor`] returned [`MoorResponse::Accepted`] for the same ship
    /// with no other call in between.
    ///
    /// Calling it without that precondition is a bug in the caller; the
    /// behavior is then implementation-defined but must not panic.
    fn can_moor(&self, ship: ShipId) -> MoorResponse; // read-only method

    fn is_moored(&self, ship: ShipId) -> bool;

    /// Frees the berth held by `ship`.
    ///
    /// The Chart calls this when a moored Ship leaves
    /// and when a moored Ship sinks. If `ship` is not moored here,
    /// this does nothing.
    fn leave_berth(&mut self, ship: ShipId);  // using a result for eventual  panics that shouldnt but might happen

    /// Creates an [`IslandResponse`] for `ship` according to the Island policies.
    ///
    /// The Chart must call this method after checking whether `ship` is moored to Island.
    fn ask_clue(&mut self, ship: ShipId) -> IslandResponse;

    /// Returns the number of rounds `ship` must wait to receive a Clue.
    ///
    /// Returns None if `ship` already has the full-strenght Clue.
    fn remaining_wait(&self, ship: ShipId) -> Option<u32>; // should just be u32 cuz it technically cannot be ever None

    fn has_more_to_offer(&self, ship: ShipId) -> bool;

    ///Updates the Island round counter with `round`.
    /// 
    /// The Chart calls this method at the beginning of every round.
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
