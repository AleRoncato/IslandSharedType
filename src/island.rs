use crate::types::{Clue, Params, Round, Seed, ShipId};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IslandResponse {
    Respond(Clue),
    Refused,
    Wait(u32),
    NothingMore //or Done
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoorResponse {
    Accepted,
    Refused,
    NoSpace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepairHullResponse{
    Accepted(u32),
    Refused,

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
    Wait(u32),
}

impl Display for MoorResponse{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self{
            MoorResponse::Accepted => write!(f,"Accepted"),
            MoorResponse::NoSpace => write!(f,"NoSpace"),
            MoorResponse::Refused => write!(f,"Refused"),
        }
    }
}

// Understand how the chart creates the islands, should there be a factory function or new is good enough

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

    fn moor(&mut self, ship: ShipId);

    fn can_moor(&self, ship: ShipId) -> MoorResponse; // read-only method

    fn leave_berth(&mut self, ship: ShipId) -> Result<(), String>;  // using a result for eventual  panics that shouldnt but might happen

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
