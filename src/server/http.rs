//
// http.rs
// Meg
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/02/2021
// 
// Copywrite (c) 2021 Wess.io
//

use std::{sync::Arc,};

use tokio_stream::StreamExt;
use tokio::task::JoinHandle;

use tide::{self, Server as TideServer};
use tide_websockets::{WebSocket, Message};

use crate::{config::{
    Config, 
    server::Server as ServerConfig
  }, console_log, string, supervisor::Supervisor};


use crate::server::js;

#[derive(Clone)]
pub struct Server {
  app:Arc<TideServer<()>>,
  config:ServerConfig
}


impl Server {
  pub fn new() -> Self {
    let config = Config::read().unwrap().server.unwrap_or(ServerConfig::default());
    let folder = config.folder.clone();
    let mut app = tide::new();

    app.at("meg.js").get(|_| async {
      let port = Config::read()
        .unwrap()
        .server
        .unwrap_or(ServerConfig::default())
        .port;

      let response = 
        tide::Response::builder(200)
        .header("Content-Type", "text/javascript")
        .body(js::websocket(port))
        .build();

      Ok(response)
    });

    app.at("/").serve_file(format!("{}/index.html", folder)).unwrap();
    app.at("/*").serve_dir(folder).unwrap();

    app.at("/dev")
    .with(WebSocket::new(|_request, mut stream| async move {
      let subscriber = Supervisor::subscribe().unwrap();

      while let Some(msg) = subscriber.next().await {
        stream.send_string(string!("reload"));
      }

      Ok(())
    }))
    .get(|_| async move { Ok("this was not a websocket request") });
    
    Self{
      app: Arc::new(app),
      config
    }
  }

  pub fn run(self) -> JoinHandle<()> {
    console_log!("Starting static http server at http://localhost:{}", self.config.port);

    let app = (*self.app).clone();

    tokio::spawn(async move {
      app.listen(
        format!("127.0.0.1:{}", self.config.port)
      ).await.unwrap();
    })
  }
}

unsafe impl Sync for Server {}
