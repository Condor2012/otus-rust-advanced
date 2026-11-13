//! ШАГ 1. Разделяемое состояние сервера: кто сейчас в комнате и куда ему писать.
//!
//! Идея, которую надо удержать в голове: соединения между собой напрямую
//! не общаются. У каждого соединения есть свой исходящий канал (`mpsc`),
//! а в общей `HashMap` лежит только отправляющий конец канала.
//! «Разослать всем» = пройти по мапе и положить сообщение в каждый канал.
//!
//! Вопрос на подумать: почему здесь `std::sync::Mutex`, а не
//! `tokio::sync::Mutex`? (Ответ — в конспекте, шаг 1.)

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use netchat_protocol::ServerMsg;
use tokio::sync::mpsc::UnboundedSender;

/// Исходящий конец очереди сообщений одного клиента.
pub type Tx = UnboundedSender<ServerMsg>;

/// Общая для всех соединений «комната».
#[derive(Debug, Default)]
pub struct Room {
    peers: HashMap<String, Tx>,
}

/// То, что реально передаётся в задачи соединений.
pub type SharedRoom = Arc<Mutex<Room>>;

pub fn shared_room() -> SharedRoom {
    Arc::new(Mutex::new(Room::default()))
}

impl Room {
    /// Зарегистрировать участника. Вернуть `false`, если ник уже занят.
    pub fn join(&mut self, _nick: &str, _tx: Tx) -> bool {
        // TODO(шаг 1): проверить занятость ника и вставить в `peers`.
        todo!("Room::join")
    }

    /// Убрать участника из комнаты.
    pub fn leave(&mut self, _nick: &str) {
        // TODO(шаг 1)
        todo!("Room::leave")
    }

    /// Отправить одному. `false` — такого ника нет (или он уже отвалился).
    pub fn send_to(&self, _nick: &str, _msg: ServerMsg) -> bool {
        // TODO(шаг 1): найти Tx и попробовать `send`.
        // Подсказка: `UnboundedSender::send` возвращает Result. Что означает Err?
        todo!("Room::send_to")
    }

    /// Разослать всем, кроме `except` (обычно — самого отправителя).
    pub fn broadcast(&self, _msg: &ServerMsg, _except: Option<&str>) {
        // TODO(шаг 1): пройти по peers, пропустить `except`, остальным — send.
        todo!("Room::broadcast")
    }

    /// Ники всех участников, отсортированные (чтобы вывод был предсказуемым).
    pub fn users(&self) -> Vec<String> {
        // TODO(шаг 1)
        todo!("Room::users")
    }

    pub fn len(&self) -> usize {
        self.peers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.peers.is_empty()
    }
}

// Эти тесты сейчас красные. Задача шага 1 — сделать их зелёными:
//     cargo test -p netchat-server --lib
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    fn peer() -> (Tx, mpsc::UnboundedReceiver<ServerMsg>) {
        mpsc::unbounded_channel()
    }

    #[test]
    fn duplicate_nick_is_rejected() {
        let mut room = Room::default();
        let (a, _ra) = peer();
        let (b, _rb) = peer();
        assert!(room.join("alice", a));
        assert!(!room.join("alice", b));
        assert_eq!(room.len(), 1);
    }

    #[test]
    fn broadcast_skips_the_sender() {
        let mut room = Room::default();
        let (a, mut ra) = peer();
        let (b, mut rb) = peer();
        room.join("alice", a);
        room.join("bob", b);

        let msg = ServerMsg::Chat { from: "alice".into(), body: "привет".into() };
        room.broadcast(&msg, Some("alice"));

        assert!(ra.try_recv().is_err());
        assert_eq!(rb.try_recv().unwrap(), msg);
    }

    #[test]
    fn send_to_unknown_nick_returns_false() {
        let room = Room::default();
        assert!(!room.send_to("nobody", ServerMsg::System { text: "?".into() }));
    }

    #[test]
    fn leave_removes_peer() {
        let mut room = Room::default();
        let (a, _ra) = peer();
        room.join("alice", a);
        room.leave("alice");
        assert!(room.is_empty());
    }
}
