use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use teloxide::{
    dispatching::dialogue::InMemStorage, prelude::*, utils::command::BotCommands,
};

#[derive(BotCommands, Clone, Debug, PartialEq)]
#[command(rename_rule = "lowercase", description = "Команды бота задач:")]
enum Command {
    #[command(description = "показать справку")]
    Help,
    #[command(description = "добавить задачу: /add купить молоко")]
    Add(String),
    #[command(description = "список задач")]
    List,
    #[command(description = "отметить выполненной: /done 1")]
    Done(usize),
    #[command(description = "добавить задачу пошагово")]
    New,
}

#[derive(Clone, Default)]
enum State {
    #[default]
    Idle,
    WaitingTitle,
    Default,
}

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type Store = Arc<Mutex<HashMap<ChatId, Vec<(String, bool)>>>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

async fn on_command(bot: Bot, msg: Message, cmd: Command, store: Store, dialogue: MyDialogue) -> HandlerResult {
    let chat = msg.chat.id;
    let text = match cmd {
        Command::Help => Command::descriptions().to_string(),
        Command::Add(title) if title.trim().is_empty() => "Напишите так: /add купить молоко".into(),
        Command::Add(title) => {
            dialogue.update(State::Default).await?;
            add_task(&store, chat, title)
        } 
        Command::List => list_tasks(&store, chat),
        Command::Done(n) => {
            let mut store = store.lock().unwrap();
            match store.get_mut(&chat).and_then(|t| t.get_mut(n.wrapping_sub(1))) {
                Some(task) => {
                    task.1 = true;
                    format!("Готово: {}", task.0)
                }
                None => format!("Задачи №{n} нет"),
            }
        }
        Command::New => {
            dialogue.update(State::WaitingTitle).await?;
            "Как назвать задачу?".into()
        }
    };
    bot.send_message(chat, text).await?;
    Ok(())
}

async fn on_echo(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, echo(msg)).await?;
    Ok(())
}

async fn on_title(bot: Bot, msg: Message, store: Store, dialogue: MyDialogue) -> HandlerResult {
    match msg.text() {
        Some(title) => {
            let reply = add_task(&store, msg.chat.id, title.to_owned());
            bot.send_message(msg.chat.id, reply).await?;
            dialogue.exit().await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Пришлите название текстом").await?;
        }
    }
    Ok(())
}

fn add_task(store: &Store, chat: ChatId, title: String) -> String {
    let mut store = store.lock().unwrap();
    let tasks = store.entry(chat).or_default();
    tasks.push((title.clone(), false));
    format!("Добавил задачу №{}: {title}", tasks.len())
}

fn list_tasks(store: &Store, chat: ChatId) -> String {
    let store = store.lock().unwrap();
    match store.get(&chat) {
        Some(tasks) if !tasks.is_empty() => tasks
            .iter()
            .enumerate()
            .map(|(i, (t, done))| format!("{}. {} {t}", i + 1, if *done { "✅" } else { "⬜" }))
            .collect::<Vec<_>>()
            .join("\n"),
        _ => "Задач пока нет".into(),
    }
}

fn echo(msg: Message) -> String {
    let text = msg.text().unwrap_or_default();
    format!("🤖 {text}")
}

fn schema() -> teloxide::dispatching::UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use teloxide::dispatching::dialogue;

    let commands = teloxide::filter_command::<Command, _>().endpoint(on_command);

    let messages = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(commands)
        .branch(dptree::case![State::WaitingTitle].endpoint(on_title))
        .branch(dptree::case![State::Default].endpoint(on_echo));

    dialogue::enter::<Update, InMemStorage<State>, State, _>().branch(messages)
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let bot = Bot::from_env(); // токен из TELOXIDE_TOKEN
    bot.set_my_commands(Command::bot_commands()).await.ok();

    let store: Store = Default::default();
    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![store, InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_commands() {
        assert_eq!(Command::parse("/add купить молоко", "tasks_bot").unwrap(), Command::Add("купить молоко".into()));
        assert_eq!(Command::parse("/done 2", "tasks_bot").unwrap(), Command::Done(2));
        assert!(Command::parse("/done два", "tasks_bot").is_err());
    }

    #[test]
    fn store_roundtrip() {
        let store: Store = Default::default();
        let chat = ChatId(1);
        assert_eq!(add_task(&store, chat, "a".into()), "Добавил задачу №1: a");
        assert!(list_tasks(&store, chat).contains("1. ⬜ a"));
    }
}
