use crate::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
/// A [Text2dBundle] with [TextBox] inserted will display the contents of the
/// [crate::resources::TextBox] resource.
pub struct TextBox;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
/// An Entity with both a Transform and a CursorMarker will be moved to the
/// cursor's location. Only one of these should exist at a time.
pub struct CursorMarker;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
pub struct BlueprintSelector;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
/// Indicates a [Text2dBundle] object that is used to show the user's username
/// on the titlescreen in the bottom left.
pub struct TitleScreenUser;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
pub struct UILocked;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
pub struct DialougeText;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Component)]
pub struct RemoveOnStateChange;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Component)]
pub struct PauseMenuMarker {
    pub type_: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Component)]
pub struct ChatBox {
    pub location: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Component)]
pub struct HotbarMarker {
    pub location: usize,
    /// Represents the type of hotbar object this is.
    /// 1 = slot background
    /// 2 = selected slot
    /// 3 = slot contents
    pub type_: usize,
}

#[derive(Clone, Copy, Debug, Component)]
pub struct SettingsPageComp {
    /// What type of component this is
    /// 0 | unimportant/misc
    /// 1 | video settings fullscreen text
    /// 2 | video settings increase scaling text
    /// 3 | video settings decrease scaling text
    /// 4 | video settings scaling text
    pub type_: u8,
}

#[derive(Clone, Debug, Eq, PartialEq, Component)]
pub struct Tile {
    pub chunk: (isize, isize),
    pub position: (usize, usize),
    pub sprite_sheet: Handle<Image>,
    pub sprite_index: usize,
}
