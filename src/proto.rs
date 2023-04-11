//! The code generated from the agones protobuffer files

pub mod api {
    pub use crate::generated::agones::dev::sdk::*;
}

#[cfg(feature = "player-tracking")]
pub mod alpha {
    pub use crate::generated::agones::dev::sdk::alpha::*;
}
