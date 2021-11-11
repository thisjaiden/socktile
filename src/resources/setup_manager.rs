pub struct SetupManager {
    pub number_of_assets: Option<usize>,
    pub internet_access: Option<bool>,
    pub ggs_access: Option<bool>
}

impl SetupManager {
    pub fn init() -> SetupManager {
        SetupManager {
            number_of_assets: None,
            internet_access: None,
            ggs_access: None
        }
    }
}
