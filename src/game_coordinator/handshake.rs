use crate::NetMessage;

pub trait GCHandshake {
    type Hello: NetMessage;

    type Welcome: NetMessage;

    fn app_id(&self) -> u32;

    fn hello(&self) -> Self::Hello;
}
