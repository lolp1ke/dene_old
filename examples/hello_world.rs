use std::sync::Arc;

use dene::{
  App, AppContext, Application, Context,
  element::{InteractiveElement, IntoElement},
  elements::div,
  executor::{BackgroundExecutor, ForegroundExecutor},
  keybind::KeybindsFile,
  style::Styled,
  view::{Interactive, Render},
  window::Window,
};
use tokio::sync::mpsc;

fn main() {
  // let rt = tokio::runtime::Builder::new_multi_thread()
  //   .enable_all()
  //   .build()
  //   .unwrap();
  // let (tx, rx) = mpsc::unbounded_channel();
  // let foreground_executor = ForegroundExecutor::new(tx);

  // let multi_thread_handle = Arc::new(rt.handle().clone());
  // let background_executor = BackgroundExecutor::new(multi_thread_handle);
  // let app = App::new(foreground_executor, background_executor);
  // rt.block_on(async {
  //   App::run(app.clone(), rx, move |cx| {
  //     let keybinds = KeybindsFile::parse(
  //       r#"
  //     [[keybindings]]
  //     [keybindings.bindings]
  //     "ctrl-q" = "Quit"
  //     "#,
  //       cx,
  //     )
  //     .unwrap();
  //     cx.load_keybinds(keybinds);

  //     cx.open_window(Default::default(), |_, cx| {
  //       cx.new_entity(|_| HelloWorld {})
  //     });
  //   })
  //   .await
  //   .unwrap();
  // });

  let app = Application::new();
  app.run(move |cx| {
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
  });
}

struct HelloWorld {}
impl Render for HelloWorld {
  fn render(
    &mut self,
    _window: &mut Window,
    cx: &mut Context<Self>,
  ) -> impl IntoElement {
    div()
      .flex()
      .flex_col()
      .gap_y(10.0)
      .items_center()
      .justify_center()
      .child(div().flex().gap_x(5.0).child("t").child("s"))
      .child("world")
      .child("one piece")
  }
}
impl Interactive for HelloWorld {}
