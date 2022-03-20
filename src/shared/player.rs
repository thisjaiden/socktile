use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct PlayerData {
    pub inventory: Inventory,
    pub stats: Stats,
    pub recipes: Recipes,
    pub achievements: Achievements
}

impl PlayerData {
    pub fn new() -> PlayerData {
        PlayerData {
            inventory: Inventory::empty(),
            stats: Stats::starting(),
            recipes: Recipes::starting(),
            achievements: Achievements::none()
        }
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Inventory {

}

impl Inventory {
    pub fn empty() -> Inventory {
        Inventory {

        }
    }
}

#[derive(Clone, Copy, PartialEq, Deserialize, Serialize, Debug)]
pub struct Stats {
    pub mining: usize,
    pub fishing: usize,
    pub cooking: usize,
    pub crafting: usize,
    pub trading: usize
    // ...ect
}

impl Stats {
    pub fn starting() -> Stats {
        Stats {
            mining: 1,
            fishing: 1,
            cooking: 1,
            crafting: 1,
            trading: 1
        }
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Recipes {
    unlocked: Vec<Recipe>,
    locked: Vec<Recipe>
}

impl Recipes {
    pub fn starting() -> Recipes {
        Recipes {
            unlocked: vec![],
            locked: vec![
                Recipe::BigRock
            ]
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
    BigRock
    // ...ect
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Achievements {
    // TODO
}

impl Achievements {
    pub fn none() -> Achievements {
        Achievements {

        }
    }
}
