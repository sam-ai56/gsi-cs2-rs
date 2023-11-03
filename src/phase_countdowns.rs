use serde::{
    Serialize, Deserialize
};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PhaseCountdowns {
    /// Phase name. (freezetime, live, bomb, defuse, over)
    pub phase: String,
    // pub phase_countdown: Option<String>,
    /// Time until the end of the phase.
    pub phase_ends_in: Option<String>
}