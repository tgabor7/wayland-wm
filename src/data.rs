use smithay::{reexports::wayland_server::Display, backend};
use crate::state::State;

pub struct Data {
  pub display: Display,
  pub state: State
}