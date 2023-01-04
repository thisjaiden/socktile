use crate::prelude::*;

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct PlayerData {
    pub inventory: Inventory,
    pub stats: Stats,
    pub recipes: Recipes,
    pub achievements: Achievements,
}

impl PlayerData {
    pub fn new() -> PlayerData {
        PlayerData {
            inventory: Inventory::empty(),
            stats: Stats::starting(),
            recipes: Recipes::starting(),
            achievements: Achievements::none(),
        }
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Inventory {
    pub selected_slot: usize,
    pub hotbar: [Option<Item>; 10],
}

impl Inventory {
    pub fn empty() -> Inventory {
        Inventory {
            selected_slot: 0,
            hotbar: [None; 10],
        }
    }
    pub fn hotbar_empty_space(&self) -> Option<usize> {
        for (index, item) in self.hotbar.iter().enumerate() {
            if item.is_none() {
                return Some(index);
            }
        }
        None
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Debug)]
pub enum Item {
    MakeshiftAxe,
    MakeshiftFishingRod,
    Blueprint,
    Wood,
}

impl Item {
    pub fn from_str(from: &str) -> Item {
        match from {
            "MakeshiftAxe" => Item::MakeshiftAxe,
            "MakeshiftFishingRod" => Item::MakeshiftFishingRod,
            "Blueprint" => Item::Blueprint,
            "Wood" => Item::Wood,
            invalid_name => {
                error!("No item with name {invalid_name}");
                panic!("{FATAL_ERROR}");
            }
        }
    }
    pub fn action(&self) -> ItemAction {
        match self {
            Item::MakeshiftAxe => ItemAction::Chop(1),
            Item::MakeshiftFishingRod => ItemAction::Fish(1),
            Item::Blueprint => ItemAction::Blueprint,
            _ => ItemAction::None,
        }
    }
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Clone, Copy, Debug)]
pub enum ItemAction {
    /// Item has no action.
    None,
    /// Item chops materials (power multiplier)
    Chop(usize),
    /// Item fishes (power multiplier)
    Fish(usize),
    /// Item digs (power multiplier)
    Dig(usize),
    /// Item mines (power multipler)
    Mine(usize),
    /// Item modifies terrain
    Blueprint,
}

#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Debug)]
pub struct Stats {
    pub mining: usize,
    pub fishing: usize,
    pub cooking: usize,
    pub crafting: usize,
    pub trading: usize, // ...ect
}

impl Stats {
    pub fn starting() -> Stats {
        Stats {
            mining: 1,
            fishing: 1,
            cooking: 1,
            crafting: 1,
            trading: 1,
        }
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Recipes {
    unlocked: Vec<Recipe>,
    locked: Vec<Recipe>,
}

impl Recipes {
    pub fn starting() -> Recipes {
        Recipes {
            unlocked: vec![],
            locked: vec![Recipe::BigRock],
        }
    }
    pub fn _unlock_all(&mut self) {
        for recipe in &self.locked {
            self.unlocked.push(*recipe);
        }
        self.locked.clear();
    }
    pub fn _is_unlocked(&mut self, recipe: Recipe) -> bool {
        self.unlocked.contains(&recipe)
    }
}

#[derive(Clone, Copy, PartialEq, Deserialize, Serialize, Debug)]

pub enum Recipe {
    BigRock, // ...ect
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Achievements {
    // TODO
}

impl Achievements {
    pub fn none() -> Achievements {
        Achievements {}
    }
}
