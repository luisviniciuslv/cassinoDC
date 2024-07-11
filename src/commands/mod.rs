pub mod par_ou_impar;
pub mod profile;
pub mod adm;
pub mod rec;

use poise::Command;

use crate::{Data, Error};

pub fn get_commands() -> Vec<Command<Data, Error>> {
    vec![
        profile::profile(),
        rec::rec(),
    ]
}
