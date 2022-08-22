// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

fn main() -> wry::Result<()> {
  use std::fs::{canonicalize, read};

  use wry::{
    application::{
      event::{Event, StartCause, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::WindowBuilder,
    },
    http::{header::CONTENT_TYPE, http::Method, Response},
    webview::WebViewBuilder,
  };

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title("Hello World")
    .build(&event_loop)
    .unwrap();

  let _webview = WebViewBuilder::new(window)
    .unwrap()
    .with_custom_protocol("wry".into(), move |mut request| {
      // Remove url scheme
      let path = request.uri().path().to_string();

      if request.method() == Method::POST {
        tokio::spawn(async move {
          let body = hyper::body::to_bytes(request.body_mut())
            .await
            .unwrap();
          let body_string = String::from_utf8_lossy(&body);
          for body in body_string.split('&') {
            println!("Value sent; {:?}", body);
          }
        });
      }

      Response::builder()
        .header(CONTENT_TYPE, "text/html")
        .body(read(canonicalize(&path)?)?.into())
        .map_err(Into::into)
    })
    // tell the webview to load the custom protocol
    .with_url("wry://examples/form.html")?
    .build()?;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => println!("Wry application started!"),
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      _ => (),
    }
  });
}
