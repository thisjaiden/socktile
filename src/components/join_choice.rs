use crate::shared::listing::GameListing;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct JoinChoice {
    pub list_index: usize,
    pub information: GameListing,
    pub outlined: bool,
    pub invite: bool
}

impl JoinChoice {
    pub fn new(index: usize, listing: GameListing, invite: bool) -> JoinChoice {
        JoinChoice {
            list_index: index,
            information: listing,
            outlined: false,
            invite
        }
    }
}
