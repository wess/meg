//
// init.rs
// Meg
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/02/2021
// 
// Copywrite (c) 2021 Wess.io
//

use clap::{App};
use crate::{config::Config, console_error, console_panic, console_success};

pub struct Init {}

impl Init {
  pub fn app() -> App<'static> {
    App::new("init")
    .about("Creates a new .megfile in current directory")  
  }

  pub fn run() {
    if Config::exists() {
      console_error!("A .megfile already exists in this directory");
      return;
    }

    let default = Config::default();

    match Config::write(&default) {
      Err(why) => console_panic!("Could not write .megfile: {}", why),
      Ok(_) => console_success!("Successfully created .megfile")
    };

  }
}
