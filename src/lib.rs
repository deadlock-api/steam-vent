#![allow(clippy::result_large_err)]

pub mod auth;
pub mod connection;
mod eresult;
mod game_coordinator;
mod message;
mod net;
mod serverlist;
mod service_method;
mod session;
mod transport;

pub use steam_vent_proto as proto;

pub use connection::{Connection, ConnectionTrait, ReadonlyConnection};
pub use eresult::EResult;
pub use game_coordinator::GameCoordinator;
pub use message::{NetMessage, UntypedMessage};
pub use net::{NetworkError, RawNetMessage};
pub use serverlist::{DiscoverOptions, ServerDiscoveryError, ServerList};
pub use session::{ConnectionError, LoginError};
