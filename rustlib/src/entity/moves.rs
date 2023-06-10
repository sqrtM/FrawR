use super::entity::{Entity};

pub trait Moves {
    fn move_up(&mut self);
    fn move_down(&mut self);
    fn move_left(&mut self);
    fn move_right(&mut self);
    fn stay_still(&mut self);
}

impl Moves for Entity {
    fn move_up(&mut self) {
        self.location.y -= 1
    }

    fn move_down(&mut self) {
        self.location.y += 1
    }

    fn move_left(&mut self) {
        self.location.x -= 1
    }

    fn move_right(&mut self) {
        self.location.x += 1
    }

    fn stay_still(&mut self) {
        ()
    }
}
