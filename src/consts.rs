/// Z-Axis for terrain and other background objects
pub const BACKGROUND: f32 = 0.0;
/// Z-Axis for players
pub const PLAYER_CHARACTERS: f32 = 50.0;
/// Z-Axis for text
pub const UI_TEXT: f32 = 101.0;
/// Z-Axis for the cursor
pub const CURSOR: f32 = 250.0;

/// Is this an internal dev build?
pub const DEV_BUILD: bool = true;
/// Should development server addresses be used?
pub const DEV_ADDRS: bool = false;
/// Allow a GGS to be run from this build?
pub const ALLOW_GGS: bool = true;

/// The current version tag for netty. If this is different from whoever you're talking to, they're likely
/// using an incompatible protocol.
pub const NETTY_VERSION: &str = "closed-alpha-iteration-13";
/// Port for network connections
pub const NETTY_PORT: &str = "11111";
/// Standard global game server address
pub const GGS: &str = "69.180.176.49:11111";
/// Global game server address for dev builds (localhost, essentially)
pub const DEV_GGS: &str = "127.0.0.1:11111";
