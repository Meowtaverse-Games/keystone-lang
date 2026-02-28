use crate::Direction;

pub trait ExternalApi {
    fn is_touched(&self) -> bool;
    fn is_empty(&self, dir: Direction) -> bool;
}
