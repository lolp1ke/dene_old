use dene::{
  AppContext, Application, Context,
  element::{InteractiveElement, IntoElement},
  elements::{Input, InputState, div},
  keybind::KeybindsFile,
  style::Styled,
  view::Render,
  window::Window,
};

fn main() {
  let app = Application::new();
  app.run(move |cx| {
    let keybinds = KeybindsFile::parse(
      r#"
      [[keybindings]]
      [keybindings.bindings]
      "ctrl-q" = "Quit"

      [[keybindings]]
      context = "input"
      [keybindings.bindings]
      "delete" = "Delete"
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
    window: &mut Window,
    cx: &mut Context<Self>,
  ) -> impl IntoElement {
    let input_state = cx.new_entity(|_| InputState::new());
    let input = cx.new_entity(|_| Input::new(input_state));

    div()
      .flex()
      .flex_col()
      .gap_y(10.0)
      .items_center()
      .justify_center()
      .on_key_down(cx.listener(|_, _, _, _| {
        print!("123");
      }))
      .child(div().flex().gap_x(5.0).child("hello").child("world"))
      .child("hi")
      .child("one piece")
      .child(input)
  }
}
