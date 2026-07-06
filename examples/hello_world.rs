use std::sync::Arc;

use dene::{
  App, AppContext, Context,
  element::IntoElement,
  elements::div,
  executor::{BackgroundExecutor, ForegroundExecutor},
  keybind::KeybindsFile,
  style::Styled,
  view::{Interactive, Render},
  window::Window,
};
use tokio::sync::mpsc;

fn main() {
  let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap();
  let (tx, rx) = mpsc::unbounded_channel();
  let foreground_executor = ForegroundExecutor::new(tx);

  let multi_thread_handle = Arc::new(rt.handle().clone());
  let background_executor = BackgroundExecutor::new(multi_thread_handle);
  let app = App::new(foreground_executor, background_executor);
  rt.block_on(async {
    App::run(app.clone(), rx, move |cx| {
      let keybinds = KeybindsFile::parse(
        r#"
      [[keybindings]]
      [keybindings.bindings]
      "ctrl-q" = "Quit"
      "#,
        cx,
      )
      .unwrap();
      cx.load_keybinds(keybinds);

      cx.open_window(Default::default(), |_, cx| {
        cx.new_entity(|_| HelloWorld {})
      });
    })
    .await
    .unwrap();
  });
}

struct HelloWorld {}
impl Render for HelloWorld {
  fn render(
    &mut self,
    _window: &mut Window,
    _cx: &mut Context<Self>,
  ) -> impl IntoElement {
    div()
      .flex()
      .flex_col()
      .gap_y(10.0)
      .child(div().child("text long"))
      .child("world")
      .child("one piece")
    // .child("hello, world!")
    // .child("next line")
  }
}
impl Interactive for HelloWorld {}
