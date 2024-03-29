use crate::prelude::*;
use crate::{
    resources::ChatMessage,
    shared::{listing::GameListing, player::Inventory},
};

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
    JoinedGame(Transform, bool),
    /// State of a user's inventory.
    /// (Inventory)
    InventoryState(Inventory),
    /// A list of online users for a given world
    /// (Array (User, Position))
    OnlinePlayers(Vec<(User, Transform)>),
    /// A client requests full data pertaining to a chunk
    /// (Chunk Location)
    RequestChunk((isize, isize)),
    /// The server sends over a chunk of terrain
    /// (Chunk Location, [Tile ID])
    ChunkData((isize, isize), Vec<usize>),
    /// An update has occurred to a single tile.
    /// Tile coordinates are world aligned (+x right, +y up) starting in the logical bottom left.
    /// (Chunk Location, Tile Location, New State)
    TileUpdate((isize, isize), (usize, usize), usize),
    /// Sends over all game objects.
    /// (Game Objects)
    AllObjects(Vec<Object>),
    /// Updates a given object.
    /// (Updated Object)
    UpdateObject(Object),
    /// Removes an object by UUID.
    /// (Object UUID)
    RemoveObject(uuid::Uuid),
    /// Creates an object on the client.
    /// (New Object)
    CreateObject(Object),
    /// Requests moving a player to a new position in a world.
    /// (New Position)
    RequestMove(Transform),
    /// Updates the position of a players who has moved.
    /// (Player, New Position)
    PlayerPositionUpdate(User, Transform),
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
    PlayerConnected(User, Transform),
    /// Sends a chat message to other players.
    /// (Message)
    SendChatMessage(ChatMessage),
    /// Recieves a chat message.
    /// (Message)
    ChatMessage(ChatMessage),
    /// Sends/Recieves an animation for a player using an item
    /// (Action)
    ActionAnimation(ItemAction),
}

impl netty::Packet for Packet {
    fn from_reader<R: std::io::Read>(reader: &mut R) -> Self {
        let maybe_pkt = bincode::deserialize_from(reader);
        match maybe_pkt {
            Ok(pkt) => {
                trace!("Got a packet {:?}!", pkt);
                pkt
            }
            Err(e) => {
                println!("Errored getting packet {:?}!", e);
                panic!();
            }
        }
    }

    fn write<W: std::io::Write + ?Sized>(&self, writer: &mut W) {
        //println!("Writing a packet {:?}!", self);
        let write_state = writer.write_all(&bincode::serialize(self).expect("Netty unable to serialize packet"));
        if write_state.is_err() {
            warn!("Netty unable to write serialized packet due to error: {:?}", write_state);
            warn!("Packet data: {:?}", self);
        }
        writer.flush().expect("Netty unable to flush buffer");
    }
    fn to_vec(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }
}
