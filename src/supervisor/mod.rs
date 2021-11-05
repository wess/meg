//
// mod.rs
// Meg
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/03/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::sync::{Arc, Mutex};
use result::MegResult;

use tokio::{
  task::JoinHandle,
  sync::mpsc::{
    unbounded_channel,
  }
};

use watch::{Watch};

use crate::{armu, config::Config, console_info, console_panic, result, server::http};
use crate::watch;


pub mod broker;

type Supervised = Arc<Mutex<JoinHandle<()>>>;

#[derive(Debug, Clone)]
pub struct Supervisor {
  config: Config,
  http: Option<Supervised>,
  watch: Option<Supervised>,
}

impl Supervisor {
  pub fn init(config:Config) -> Self {
    Self {
      config,
      http: None,
      watch: None,
    }
  }
  
  pub fn start_http(&mut self) {
    self.stop_http();

    let server = http::Server::new();

    self.http = Some(Arc::new(
      Mutex::new(
        server.run()
      )
    ));
  }

  pub fn stop_http(&mut self) {
    if let Some(child) = &self.http {
      child.lock().unwrap().abort();

      self.http = None;
    }
  }

  pub async fn start_watch(&mut self) {
    self.stop_watch();

    let cfg = match self.config.clone().watch {
      Some(c) => c,
      None => console_panic!("Watch is not configured in .megfile")
    };

    let (tx, mut rx) = unbounded_channel::<String>();
    let watch_tx = tx.clone();

    let mut watch = Watch::new(watch_tx);

    self.watch = Some(armu!(watch.run()));

    if cfg.server == true {
      self.start_http();
    }

    loop {
      match rx.recv().await {
        Some(_res) => {
          if cfg.verbose {
            console_info!("Restarting...");
          }
        },
        None => {}
      }
    }
  }


  pub fn stop_watch(&mut self) {
    if let Some(child) = &self.watch {
      child.lock().unwrap().abort();

      self.watch = None;
    }
  }


  pub async fn run(&mut self) {
    loop {}
  }
}



