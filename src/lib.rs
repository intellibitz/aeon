pub mod error;
pub mod daemon;
pub mod gawd;
pub mod gemi;
pub mod gmcp;
pub mod native;
pub mod sandbox;

pub const AEON_VERSION: &str = env!("CARGO_PKG_VERSION");
