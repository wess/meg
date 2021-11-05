//
// lib.rs
// Meg
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/02/2021
// 
// Copywrite (c) 2021 Wess.io
//

use clap::{
  App, 
  AppSettings
};

pub mod macros;
pub mod console;
pub mod result;
pub mod config;
pub mod commands;
pub mod server;
pub mod watch;
pub mod supervisor;

use result::MegResult;
use config::Config;
use supervisor::Supervisor;

use commands::{
  init::Init,
  trigger::Trigger,
};

pub async fn run() ->  MegResult<()> {
  let config = Config::read().unwrap();
  Config::load_env_vars(&config);

  let mut sup = Supervisor::init(config);

  let matches = App::new("Meg")
    .version("0.0.1")
    .author("Wess Cope <me@wess.io>")
    .about("Shaddup meg...")
    .setting(AppSettings::AllowExternalSubcommands)
    .subcommand(Init::app())
    .get_matches();

  // First we match
  match matches.subcommand_name() {
    Some("init") => Init::run(),
    Some("serve") => {
      sup.start_http(); 
    },
    Some("watch") => {
      sup.start_watch().await;
    },
    Some(cmd) => Trigger::run(string!(cmd)),
    None => println!("NO COMMANND"),
  }

  if matches.subcommand_matches("serve").is_some() || matches.subcommand_matches("watch").is_some() {
    sup.run().await;
  }

  Ok(())
}