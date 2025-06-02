# How To Use
If you haver used egui, this drag and drop system will seem familiar to you. If you have used other existing drag and drop options, this one will seem backwards.

## Your State
Your state must contain a field of type `&'a iced_drag::DragAndDrop`. This is the global, internally mutable state of the drag and drop system. Draggables and drop zones use this field in order to figure out what you're dragging and dropping.

Your drag and drop zones need Payload types. Thanks to `Any` nonsense, they don't even have to be the same type.

You might want message for each of your Payload types, in order to alter the state after a drag and drop operation.

```rust
struct State {
  dragndrop: DragAndDrop,
}

enum Message {
  Dropped(Payload)
}

enum Payload {
  Red, Green, Blue, Black,
}
```

## Draggables
This crate offers the `drag` helper function. You provide it three things: An id, a reference to the drag and drop state, and the visual elements of the draggable. You give it a payload after creating it.

```rust
let zone = drag("an_id", &state.dragndrop, "Drag Me!").payload(Payload::Blue);
```

The id is important, because it's how iced can know that two draggables are truly different ones.

## Drop Zones
The `drop_zone` helper function creates drop zones. You provide it two things: A reference to the drag and drop state, and the visual elements of the drop zone.

You must also provide it an `on_drop` function - a function that takes a Payload and outputs a Message.

```rust
let zone = drop_zone(&state.dragndrop, "Drop Here!").on_drop(Message::DropFinished);
```

## The Data Flow
In this drag and drop system, you don't care about either which widget has been dragged, or where it has been dropped. You care about intermediary data.

`iced_drag` makes the assumption that your drag and drop state is defined entirely by the data that your UI represents.

Consider a card game where you can drag cards from your hand to a space on the board. This means you have a State containing your hand and these board spaces.

You create a draggable for each card in hand. Each draggable will have a payload representing its position in hand. If there are more zones you can drag cards from, your payload may need to represent this as well (or you may have a payload type for the hand, and a different one for the other zones).

When the card is dragged somewhere, you send a message containing the payload that represented the position of your card, and use that information to update the state of the game.
