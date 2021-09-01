//! The code generated from the agones protobuffer files

pub mod api {
    // The original protobuf files have some unfortunate comment markup
    #![allow(rustdoc::broken_intra_doc_links)]

    include!("generated/agones.dev.sdk.rs");
}

#[cfg(feature = "player-tracking")]
pub mod alpha {
    include!("generated/agones.dev.sdk.alpha.rs");
}
