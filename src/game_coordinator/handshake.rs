use crate::NetMessage;

pub trait GCHandshake {
    const APP_ID: u32;

    type Hello: NetMessage;

    type Welcome: NetMessage;

    fn hello(&self) -> Self::Hello;
}
