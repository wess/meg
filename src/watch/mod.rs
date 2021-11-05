//
// mod.rs
// Meg
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/10/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::{
  fs, 
  time::Duration,
  path::PathBuf, 
  sync::{
    mpsc,
  }
};

use glob::glob;

use notify::{Watcher, RecursiveMode, watcher, DebouncedEvent};

use tokio::{
  task::JoinHandle,
  sync::mpsc::UnboundedSender,
};

use crate::{config::{
    Config, 
    watch::{Watch as WatchConfig},
  }, console_info, console_log, console_panic, string};

mod handle;
use handle::Handle;

mod trigger;
use trigger::Trigger;

type Registry = Vec<Handle>;

pub struct Watch {
  config:WatchConfig,
  registry:Registry,
  sender:UnboundedSender<String>,
}

impl Watch {
  pub fn new(sender:UnboundedSender<String>) -> Self {
    let watch_config = Config::read().unwrap().watch.clone().unwrap();

    Self {
      config: watch_config,
      registry: Vec::new(),
      sender,
    }
  }
  
  pub fn run(&mut self) -> JoinHandle<()> {
    self.registry = Self::build_registry(&self.config);

    let config = self.config.clone();
    let registry = self.registry.clone();
    let sender = self.sender.clone();

    tokio::spawn(async move {
      console_log!("Starting watch process");
      Self::process(config, registry, sender);
    })
  }
 
  fn process(config:WatchConfig, registry:Registry, sender:UnboundedSender<String>) {
    
    let cfg = config.clone();
    let reg = registry.clone();

    let (tx, rx) = mpsc::channel();
    let mut watcher = watcher(tx, Duration::from_millis(500)).unwrap();

    let mut paths:Vec<PathBuf> = Vec::new();

    if cfg.verbose {
      console_log!("Watching files:");
    }
    
    for handle in reg.iter() {
      for path in handle.paths.iter() {
        if cfg.verbose {
          console_log!("- {}", path.display());
        }

        paths.push(path.clone());        
        watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
      }
    }

    let ex_sender = sender.clone();

    loop { 
      match rx.recv() {
        Ok(event) => {
          match event {
            DebouncedEvent::NoticeRemove(_) => {},
            DebouncedEvent::NoticeWrite(_) => {},
            DebouncedEvent::Create(path) => Self::exec_all(path, &reg, ex_sender.clone()),
            DebouncedEvent::Write(path) => Self::exec_all(path, &reg, ex_sender.clone()),
            DebouncedEvent::Chmod(path) => Self::exec_all(path, &reg, ex_sender.clone()),
            DebouncedEvent::Remove(path) => Self::exec_all(path, &reg, ex_sender.clone()),
            DebouncedEvent::Rename(_, to) => Self::exec_all(to, &reg, ex_sender.clone()),
            DebouncedEvent::Error(err, _) => console_panic!("Watch error {:?}", err),
            _ => {}
          }
        },
        Err(e) => console_panic!("Watch error: {:?}", e),
      }
    }
  }
   
  fn build_registry(config:&WatchConfig) -> Vec<Handle> {
    let mut registry:Vec<Handle> = Vec::new();

    for pattern in config.patterns.clone() {
      let entries:Vec<PathBuf> = 
        glob(pattern.pattern.as_str())
        .unwrap()
        .into_iter()
        .map(|res| fs::canonicalize(res.unwrap()).unwrap())
        .collect();

      let commands:Vec<Handle> =
        pattern.commands
        .iter()
        .map(|cmd| Handle{command: cmd.clone(), paths: entries.clone()})
        .collect();

      for command in commands {
        registry.push(command);
      }
    }

    registry
  }

  fn exec(cmd:&String) {
    Trigger::new(cmd.clone()).exec();
  }

  fn exec_all(path:PathBuf, registry:&Registry, sender:UnboundedSender<String>) {
    let mut commands:Vec<String> = Vec::new();
 
    console_info!("Running tasks for changed file: {}", path.display());

    for handle in registry.iter() {
      if handle.paths.contains(&path) {
        commands.push(handle.command.clone());
      }
    }

    commands.sort();
    commands.dedup();
    commands.iter().for_each(Self::exec);

    sender.send(string!("watch.fired")).unwrap();
  }

}