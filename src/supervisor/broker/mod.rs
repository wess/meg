//
// mod.rs
// Meg
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/14/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::{collections::HashMap, sync::{Arc, Mutex, MutexGuard}};


#[derive(Clone)]
pub struct Broker {
  pub registry: Arc<Mutex<HashMap<String, String>>>,
}

impl Broker {
  pub fn new() -> Self {
    let registry = Arc::new(Mutex::new(HashMap::new()));

    Self {
      registry,
    }
  }
  
  pub fn clone(broker:  &Broker) -> Arc<Mutex<HashMap<String, String>>> {
    Arc::clone(
      &broker.registry
    )
  }

  fn list_topics(&mut self) -> Vec<String> {
    self.topics()
    .keys()
    .map(|t| t.clone())
    .collect::<Vec<String>>()
  }

  fn topics(&mut self) -> MutexGuard<HashMap<String, String>> {
    self.registry.lock().unwrap()
  }
}
