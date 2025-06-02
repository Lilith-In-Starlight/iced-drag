use iced::{Element, widget::row};
use iced_drag::{DragAndDrop, drag, drop_zone};

fn main() {
    iced::application(State::default, update, view).run();
}

#[derive(Default)]
struct State {
    dragndrop: DragAndDrop,
}

#[derive(Debug, Clone)]
enum Message {
    Dropped(Payload),
}

#[derive(Debug, Clone)]
enum Payload {
    Red,
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Dropped(payload) => println!("{payload:#?}"),
    }
}

fn view(state: &State) -> Element<Message> {
    let a = drag("uwu".to_owned(), &state.dragndrop, "Uwu").payload(Payload::Red);
    let b = drop_zone(&state.dragndrop, "owo").on_drop(Message::Dropped);

    row![a, b].into()
}
