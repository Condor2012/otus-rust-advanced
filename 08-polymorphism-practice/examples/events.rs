use polymorphism_practice::events::{Event, EventBus, EventKind, handle_once, process_stream};

fn main() {
    handle_once(Event::new(EventKind::Click, "button-1"), |event| {
        println!("Обработано: {:?} — {}", event.kind, event.payload);
    });

    let mut click_count = 0;
    let mut log = Vec::new();
    process_stream(
        vec![
            Event::new(EventKind::Click, "btn-1"),
            Event::new(EventKind::Hover, "menu"),
            Event::new(EventKind::Click, "btn-2"),
        ],
        |event| {
            if event.kind == EventKind::Click {
                click_count += 1;
            }
            log.push(format!("{:?}", event.kind));
        },
    );
    println!("Кликов: {click_count}");
    println!("Лог: {log:?}");

    let mut bus = EventBus::new();
    bus.on("click", |event| println!("Click: {}", event.payload));
    bus.on("submit", |event| println!("Submit: {}", event.payload));
    bus.emit("click", "button-1");
    bus.emit("submit", "form-data");
}
