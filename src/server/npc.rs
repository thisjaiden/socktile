use crate::prelude::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Npc {
    /// Who is this NPC?
    who: Who,
    /// Does this NPC have a popup above its head indicating that it is interactable?
    popup: bool,
    /// What memories does this NPC find important?
    memories: Vec<Memory>,
    /// TODO: better data type
    /// What is this NPC's relationship with others?
    relationships: Vec<(Person, f32)>,
    /// What is this NPC currently doing?
    current_task: Task,
    /// What are the followups to what this NPC is doing?
    /// These are mostly extensions of `current_task`, not other tasks
    queued_tasks: Vec<Task>,
    /// Where does this NPC call home?
    home_location: GridPosition
    // TODO: changed task prefrences
    // TODO: mood data bloq
}

impl Npc {
    pub fn from_name_str(from: &str) -> Npc {
        let who = Who::from_str(from);
        Npc {
            who,
            popup: false,
            memories: vec![],
            relationships: vec![],
            current_task: Task::Rest(std::time::Duration::from_secs(1)),
            queued_tasks: vec![],
            home_location: who.get_inital_home()
        }
    }
    pub fn active_popup(&self) -> bool {
        self.popup
    }
    pub fn start_popup(&mut self) {
        self.popup = true;
    }
    pub fn stop_popup(&mut self) {
        self.popup = false;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Who {
    ZinDavidson,
    AnzhelaBristlesquack,
    CoraRanlor,
    ThomasKontos
}

impl Who {
    fn from_str(from: &str) -> Who {
        match from {
            "Zin Davidson" => Who::ZinDavidson,
            "Anzhela Bristlesquack" => Who::AnzhelaBristlesquack,
            "Cora Ranlor" => Who::CoraRanlor,
            "Thomas Kontos" => Who::ThomasKontos,
            invalid_name => {
                error!("No NPC with name {invalid_name}");
                panic!("{FATAL_ERROR}");
            }
        }
    }
    fn get_inital_home(&self) -> GridPosition {
        match self {
            _ => {
                warn!("TODO: home position for {:?}", self);
                GridPosition {
                    chunk: (0, 0),
                    tile: (0, 0)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
enum Person {
    Npc(Who),
    Player(User)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Memory {
    pub task: Task,
    pub person: Person
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
struct GridPosition {
    pub chunk: (isize, isize),
    pub tile: (usize, usize)
}

/// Represents one thing an NPC could do
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum Task {
    /// Go to a location
    Travel(GridPosition),
    /// Wait some period of time
    Rest(std::time::Duration),
    /// Wander and find things
    Explore,
    /// Converse with someone
    Talk,
    /// Make or destroy something
    Change(GridPosition, ChangeType)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
enum ChangeType {
    // Chop a tree
    Chop,
    // Plant flowers/sapling
    Plant,
    // Pick flowers
    Pick,
    // Build a path
    Path,
    // Fence off an area
    Fence
}
