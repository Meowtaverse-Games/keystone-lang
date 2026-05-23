use crate::Direction;

pub trait ExternalApi {
    fn is_touched(&self) -> bool;
    fn is_empty(&self, dir: Direction) -> bool;
    fn send_signal(&self, channel: &str);
    fn receive_signal(&self, channel: &str) -> bool;
}
