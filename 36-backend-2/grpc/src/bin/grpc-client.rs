use tasks_grpc::pb::task_service_client::TaskServiceClient;
use tasks_grpc::pb::{CreateTaskRequest, GetTaskRequest, WatchTasksRequest};
use tonic::Code;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = TaskServiceClient::connect("http://127.0.0.1:50051").await?;

    // подписываемся на стрим в отдельной задаче
    let mut watcher = client.clone();
    let watch = tokio::spawn(async move {
        let mut stream = watcher.watch_tasks(WatchTasksRequest {}).await?.into_inner();
        for _ in 0..2 {
            if let Some(task) = stream.message().await? {
                println!("[watch] new task: {task:?}");
            }
        }
        Ok::<_, tonic::Status>(())
    });
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    for title in ["Изучить tonic", "Написать proto"] {
        let task = client
            .create_task(CreateTaskRequest { title: title.into() })
            .await?
            .into_inner();
        println!("created: {task:?}");
    }

    let err = client.get_task(GetTaskRequest { id: 42 }).await.unwrap_err();
    assert_eq!(err.code(), Code::NotFound);
    println!("get 42: {:?} / {}", err.code(), err.message());

    let err = client
        .create_task(CreateTaskRequest { title: " ".into() })
        .await
        .unwrap_err();
    println!("empty title: {:?} / {}", err.code(), err.message());

    watch.await??;
    Ok(())
}
