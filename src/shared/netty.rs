use crate::{
    components::GamePosition,
    shared::{
        object::Object,
        saves::User,
        terrain::TerrainState,
        listing::GameListing,
        player::Inventory
    }, resources::ChatMessage
};

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub enum Packet {
    /// Post the version of the network protocol being used.
    /// A response is expected, regardless of the version on either end.
    /// (Netty Version)
    NettyVersion(String),
    /// The server appears to use the same version. Continue.
    /// (No Data)
    AllSet,
    /// The server is running a newer version. Exit.
    /// (Server's Version)
    WrongVersion(String),
    /// Data was recieved but unable to be deserizalized.
    /// (This occurs on data courruption or a disconnect, usually the latter.)
    /// (No Data)
    FailedDeserialize,
    /// Create a user on the remote server.
    /// (User)
    CreateUser(User),
    /// Confirm creation of a profile.
    /// NOTE: The user's tag of the User in this profile should be different from that of the requested
    /// profile. Take this into account.
    /// (User)
    CreatedUser(User),
    /// Unable to create this profile due to too many users existing with this username.
    /// Usually the correct course of action is to inform the user and get a different username.
    /// (No Data)
    OverusedName,
    /// Lets the server know what user is associated with what IP.
    /// (User)
    UserPresence(User),
    /// Create a world on the remote server.
    /// (World Name)
    CreateWorld(String),
    /// Confirm creation of a world.
    /// (World ID)
    CreatedWorld(usize),
    /// Request to join a world.
    /// (World ID)
    JoinWorld(usize),
    /// Request avalable servers for the sending user.
    /// (No Data)
    AvalableServers,
    /// Sends back a list of servers.
    /// (Array (Server))
    ServerList(Vec<GameListing>),
    /// Disconnects from a world.
    /// (No Data)
    LeaveWorld,
    /// A client has been connected. Send them their position.
    /// (Player Position, Owns Server)
    JoinedGame(GamePosition, bool),
    /// State of a user's inventory.
    /// (Inventory)
    InventoryState(Inventory),
    /// A list of online users for a given world
    /// (Array (User, Position))
    OnlinePlayers(Vec<(User, GamePosition)>),
    /// The server sends over the information relating to some terrain.
    /// (Chunk, Array (Tile, Tile Override))
    ChangesChunk((isize, isize), Vec<(usize, usize, TerrainState)>),
    /// An update has occurred in a chunk.
    /// (Chunk, Tile, Tile Override)
    ChunkUpdate((isize, isize), (usize, usize), TerrainState),
    /// Sends over all game objects.
    /// (Game Objects)
    AllObjects(Vec<Object>),
    /// Updates a given object on the client.
    /// (Updated Object)
    UpdateObject(Object),
    /// Removes an object on the client by UUID.
    /// (Object UUID)
    RemoveObject(uuid::Uuid),
    /// Creates an object on the client.
    /// (New Object)
    CreateObject(Object),
    /// Requests moving a player to a new position in a world.
    /// (New Position)
    RequestMove(GamePosition),
    /// Updates the position of a players who has moved.
    /// (Player, New Position)
    PlayerPositionUpdate(User, GamePosition),
    /// A player has disconnected.
    /// (User)
    PlayerDisconnected(User),
    /// Requests to add this server to a players's list of joinable servers.
    /// (User)
    WhitelistUser(User),
    /// You don't have permission to whitelist players on this server!
    /// (No Data)
    NoWhitelistPermission,
    /// This user can't be whitelisted. Most likely they are not a real user.
    /// (No Data)
    UnwhitelistableUser,
    /// The user was whitelisted!
    /// (No Data)
    Whitelisted,
    /// A user has joined the game.
    /// (User, Initial Position)
    PlayerConnected(User, GamePosition),
    /// Sends a chat message to other players.
    /// (Message)
    SendChatMessage(ChatMessage),
    /// Recieves a chat message.
    /// (Message)
    ChatMessage(ChatMessage)
}

impl Packet {
    pub fn from_read<R: std::io::Read>(read: &mut R) -> Packet {
        bincode::deserialize_from(read).unwrap_or(Packet::FailedDeserialize)
    }
    pub fn to_write<W: std::io::Write>(write: &mut W, packet: Packet) {
        write.write_all(&bincode::serialize(&packet).expect("Netty unable to serialize packet")).expect("Netty unable to write serialized packet");
        write.flush().expect("Netty unable to flush buffer");
    }
}
