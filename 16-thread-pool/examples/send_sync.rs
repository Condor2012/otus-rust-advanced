use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

fn main() {
    // Структуры с `Send + Sync` полями сами реализуют `Send + Sync`.
    assert_send::<BothStruct>();
    assert_sync::<BothStruct>();

    // `RefCell<T>: Send`, если `T: Send`, но никогда не реализует `Sync`.
    assert_send::<RefCell<()>>();

    // Следующие проверки намеренно оставлены в комментариях: это негативные
    // примеры, и раскомментирование любой из них должно вызвать ошибку компиляции.
    //
    // assert_sync::<RefCell<()>>();       // RefCell<()>: !Sync
    // assert_send::<&RefCell<()>>();      // &RefCell<()>: !Send
    // assert_sync::<&RefCell<()>>();      // &RefCell<()>: !Sync
    // assert_send::<Rc<()>>();            // Rc<()>: !Send
    // assert_sync::<Rc<()>>();            // Rc<()>: !Sync
    // assert_send::<NeitherStruct>();     // PhantomData<Rc<()>> запрещает Send
    // assert_sync::<NeitherStruct>();     // и Sync

    println!("Положительные проверки прошли; негативные примеры — в комментариях.");
}

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

#[derive(Default)]
struct BothStruct {
    _number: u32,
    _array: [u8; 10],
    _string: String,
    _map: HashMap<u64, Vec<u8>>,
}

#[derive(Default)]
#[allow(dead_code)]
struct NeitherStruct {
    _p: PhantomData<Rc<()>>,
}
