use std::sync::{Arc, Mutex};

use async_graphql::{
    Context, ID, Object, Schema, SimpleObject, Subscription,
    http::GraphiQLSource,
};
use async_graphql_axum::{GraphQL, GraphQLSubscription};
use axum::{Router, response::Html, routing::get};
use futures::Stream;
use tokio::sync::broadcast;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

#[derive(SimpleObject, Clone)]
struct Task {
    id: ID,
    title: String,
    done: bool,
}

#[derive(Clone)]
struct Store {
    tasks: Arc<Mutex<Vec<Task>>>,
    events: broadcast::Sender<Task>,
}

struct Query;

#[Object]
impl Query {
    /// Список задач, опционально с фильтром по статусу
    async fn tasks(&self, ctx: &Context<'_>, done: Option<bool>) -> Vec<Task> {
        let store = ctx.data_unchecked::<Store>();
        store
            .tasks
            .lock()
            .unwrap()
            .iter()
            .filter(|t| done.is_none_or(|d| t.done == d))
            .cloned()
            .collect()
    }

    async fn task(&self, ctx: &Context<'_>, id: ID) -> Option<Task> {
        let store = ctx.data_unchecked::<Store>();
        store.tasks.lock().unwrap().iter().find(|t| t.id == id).cloned()
    }
}

struct Mutation;

#[Object]
impl Mutation {
    async fn add_task(&self, ctx: &Context<'_>, title: String) -> async_graphql::Result<Task> {
        if title.trim().is_empty() {
            return Err("title must not be empty".into());
        }
        let store = ctx.data::<Store>()?;
        let task = {
            let mut tasks = store.tasks.lock().unwrap();
            let task = Task { id: ID::from(tasks.len() + 1), title, done: false };
            tasks.push(task.clone());
            task
        };
        let _ = store.events.send(task.clone());
        Ok(task)
    }

    async fn complete_task(&self, ctx: &Context<'_>, id: ID) -> Option<Task> {
        let store = ctx.data_unchecked::<Store>();
        let mut tasks = store.tasks.lock().unwrap();
        let task = tasks.iter_mut().find(|t| t.id == id)?;
        task.done = true;
        Some(task.clone())
    }
}

struct Subscription;

#[Subscription]
impl Subscription {
    async fn task_added(&self, ctx: &Context<'_>) -> impl Stream<Item = Task> {
        let rx = ctx.data_unchecked::<Store>().events.subscribe();
        BroadcastStream::new(rx).filter_map(|r| r.ok())
    }
}

type TasksSchema = Schema<Query, Mutation, Subscription>;

fn schema() -> TasksSchema {
    let (events, _) = broadcast::channel(64);
    let store = Store { tasks: Default::default(), events };
    Schema::build(Query, Mutation, Subscription)
        .data(store)
        .limit_depth(5) // защита от слишком глубоких запросов
        .finish()
}

async fn graphiql() -> Html<String> {
    Html(
        GraphiQLSource::build()
            .endpoint("/")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

#[tokio::main]
async fn main() {
    let schema = schema();
    std::fs::write("schema.graphql", schema.sdl()).unwrap();

    let app = Router::new()
        .route("/", get(graphiql).post_service(GraphQL::new(schema.clone())))
        .route_service("/ws", GraphQLSubscription::new(schema));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("GraphiQL: http://localhost:8000");
    axum::serve(listener, app).await.unwrap();
}
