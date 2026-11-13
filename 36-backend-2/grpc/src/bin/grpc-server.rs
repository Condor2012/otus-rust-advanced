use std::pin::Pin;
use std::sync::{Arc, Mutex};

use tasks_grpc::pb::task_service_server::{TaskService, TaskServiceServer};
use tasks_grpc::pb::{
    CreateTaskRequest, GetTaskRequest, ListTasksRequest, ListTasksResponse, Task,
    WatchTasksRequest,
};
use tokio::sync::broadcast;
use tokio_stream::{Stream, StreamExt, wrappers::BroadcastStream};
use tonic::{Request, Response, Status, transport::Server};

#[derive(Clone)]
struct Tasks {
    store: Arc<Mutex<Vec<Task>>>,
    events: broadcast::Sender<Task>,
}

#[tonic::async_trait]
impl TaskService for Tasks {
    async fn create_task(&self, req: Request<CreateTaskRequest>) -> Result<Response<Task>, Status> {
        let title = req.into_inner().title;
        if title.trim().is_empty() {
            return Err(Status::invalid_argument("title must not be empty"));
        }
        let task = {
            let mut store = self.store.lock().unwrap();
            let task = Task { id: store.len() as i64 + 1, title, done: false };
            store.push(task.clone());
            task
        };
        let _ = self.events.send(task.clone()); // нет подписчиков — не ошибка
        Ok(Response::new(task))
    }

    async fn get_task(&self, req: Request<GetTaskRequest>) -> Result<Response<Task>, Status> {
        let id = req.into_inner().id;
        self.store
            .lock()
            .unwrap()
            .iter()
            .find(|t| t.id == id)
            .cloned()
            .map(Response::new)
            .ok_or_else(|| Status::not_found(format!("task {id} not found")))
    }

    async fn list_tasks(
        &self,
        _req: Request<ListTasksRequest>,
    ) -> Result<Response<ListTasksResponse>, Status> {
        let tasks = self.store.lock().unwrap().clone();
        Ok(Response::new(ListTasksResponse { tasks }))
    }

    type WatchTasksStream = Pin<Box<dyn Stream<Item = Result<Task, Status>> + Send>>;

    async fn watch_tasks(
        &self,
        _req: Request<WatchTasksRequest>,
    ) -> Result<Response<Self::WatchTasksStream>, Status> {
        let stream = BroadcastStream::new(self.events.subscribe())
            .map(|r| r.map_err(|e| Status::data_loss(e.to_string())));
        Ok(Response::new(Box::pin(stream)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (events, _) = broadcast::channel(64);
    let svc = Tasks { store: Default::default(), events };

    let addr = "0.0.0.0:50051".parse()?;
    println!("gRPC server on {addr}");
    Server::builder()
        .add_service(TaskServiceServer::new(svc))
        .serve(addr)
        .await?;
    Ok(())
}
