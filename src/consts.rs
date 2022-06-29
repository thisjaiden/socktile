/// Z-Axis for terrain and other background objects
pub const BACKGROUND: f32 = 0.0;
/// Z-Axis for players and NPCs
pub const PLAYER_CHARACTERS: f32 = 50.0;
/// Z-Axis for objects below the player vertically
pub const FRONT_OBJECTS: f32 = 51.0;
/// Z-Axis for ui images
pub const UI_IMG: f32 = 100.0;
/// Z-Axis for text
pub const UI_TEXT: f32 = 101.0;
/// Z-Axis for the cursor
pub const CURSOR: f32 = 250.0;
/// Z-Axis for debug lines
pub const DEBUG: f32 = 400.0;

/// Is this an internal dev build?
pub const DEV_BUILD: bool = true;
/// Allow a GGS to be run from this build?
pub const ALLOW_GGS: bool = true;
/// Show debug lines over UI hitboxes?
pub const UI_DEBUG: bool = true;
/// Should assets be included in the exe?
pub const EMBED_ASSETS: bool = true;

/// The current version tag for netty. If this is different from whoever you're talking to, they're likely
/// using an incompatible protocol.
pub const NETTY_VERSION: &str = "closed-alpha-iteration-20";
/// Port for network connections
pub const NETTY_PORT: u16 = 11111;
/// Global game server address
#[cfg(debug_assertions)]
pub const GGS: [u8; 4] = [127, 0, 0, 1];
#[cfg(not(debug_assertions))]
pub const GGS: [u8; 4] = [69, 180, 176, 49];
/// Time in seconds before a connection is considered unable to connect
pub const TIMEOUT_DURATION: u64 = 3;
/// Time in ms between game ticks on the server
pub const TICK_TIME: u64 = 25;
/// Time in minutes between game saves on the server
pub const SAVE_TIME: u64 = 30;

/// The distance before an item on the ground is drawn to players
pub const ITEM_MAGNET_DISTANCE: f32 = 256.0;
/// The distance before an item on the ground is picked up by players
pub const ITEM_PICKUP_DISTANCE: f32 = 16.0;
/// The offset between the cursor's render location and actual location
pub const CURSOR_OFFSET: [f32; 2] = [-25.0, 31.0];
/// Width of a chunk
pub const CHUNK_WIDTH: usize = 30;
/// Height of a chunk
pub const CHUNK_HEIGHT: usize = 17;
/// Amount of tiles in a chunk
pub const CHUNK_SIZE: usize = CHUNK_WIDTH * CHUNK_HEIGHT;

/// The message used for panic!s when a non-recoverable error occurs
pub const FATAL_ERROR: &str = "A fatal error occured and socktile cannot continue";
