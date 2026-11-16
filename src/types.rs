pub type ShipId = u8;

pub type Round = u32;

pub type Seed = u32;

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub struct Clue {
    pub id: u32,
}

#[non_exhaustive]
#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub archipelago_radius: u32,
    pub island_count: u8,
    pub cliff_count: u16,
    pub rock_count: u16,
    pub vision_range: u32,
    pub bombard_range: u32,
    pub hull_strength: u32,
    pub round_cap: Round,
    pub round_deadline_ms: u32,
}

impl Default for Params {
    fn default() -> Self {
        Self{
            archipelago_radius: 16,
            island_count: 4,
            bombard_range: 3,
            cliff_count: 5,
            hull_strength: 1000,
            rock_count: 3,
            round_cap: 200,
            round_deadline_ms: 100,
            vision_range: 4
        }
    }
}