use smithay::reexports::nix::sys::timerfd::ClockId;


pub struct State {
    pub clock: ClockId,
}