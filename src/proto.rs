//! The code generated from the agones protobuffer files

pub mod api {
    include!("generated/agones.dev.sdk.rs");
}

#[cfg(feature = "player-tracking")]
pub mod alpha {
    include!("generated/agones.dev.sdk.alpha.rs");
}
