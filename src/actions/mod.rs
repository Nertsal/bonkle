use specs::{Entity, World};

mod follow;
mod shoot;

pub use follow::FollowAction;
pub use shoot::ShootAction;

pub trait Action: Sync + Send {
    fn perform(self: Box<Self>, world: &mut World, actor: Entity);
}
