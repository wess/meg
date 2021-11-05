//
// js.rs
// Meg
// 
// Author: Wess Cope (me@wess.io)
// Created: 06/07/2021
// 
// Copywrite (c) 2021 Wess.io
//

pub fn websocket(port:u64) -> String {
  let template = r#"
  (function() {
    console.log("Server restarted.");

    var socket = new WebSocket('ws://localhost:%{PORT}/dev');

    socket.onmessage = function(evt) {
      if(!window.location.reload) 
        window.location.href = window.location.href;
      else
        window.location.reload()
    }
  })();
  "#;

  template.replace("%{PORT}", port.to_string().as_str())
}
