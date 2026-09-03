//! The `feed` slice reuses `posts::dto::PostView` as its item type, so there is
//! no feed-specific wire struct. This re-export keeps imports tidy and signals
//! the shared contract.

pub use crate::features::posts::dto::PostView;
