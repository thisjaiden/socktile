use super::*;

#[derive(Clone)]
pub struct Globals {
    pub worlds: Vec<SaveGame>,
    pub profiles: Vec<Profile>,
    pub user_to_addr: HashMap<User, SocketAddr>,
    pub addr_to_user: HashMap<SocketAddr, User>,
    pub user_to_world: HashMap<User, usize>,
    pub last_autosave: std::time::Instant,
}

impl Default for Globals {
    fn default() -> Self {
        let saves = saves();
        let mut sorted = vec![];
        for i in 0..saves.len() {
            for save in saves.clone() {
                if save.internal_id == i {
                    sorted.push(save);
                }
            }
        }
        Self {
            worlds: sorted,
            profiles: profiles(),
            user_to_addr: default(),
            addr_to_user: default(),
            user_to_world: default(),
            last_autosave: std::time::Instant::now(),
        }
    }
}
