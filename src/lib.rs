use seed::prelude::*;

pub mod client;
pub mod config;
pub mod error;
pub mod login;
pub mod video;
pub mod response;

pub  mod  prelude {
    pub  use crate::{client::*, config::*, error::*, login::prelude::*, video::prelude::*};
}

