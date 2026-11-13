use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, postgres::PgPoolOptions};
use tower_http::trace::TraceLayer;

// ---------- модели ----------
#[derive(Serialize, FromRow)]
struct Task {
    id: i64,
    title: String,
    done: bool,
}

#[derive(Deserialize)]
struct NewTask {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTask {
    title: Option<String>,
    done: Option<bool>,
}

#[derive(Deserialize)]
struct ListParams {
    done: Option<bool>,
    #[serde(default = "default_limit")]
    limit: i64,
}
fn default_limit() -> i64 {
    20
}

// ---------- состояние ----------
#[derive(Clone)]
struct AppState {
    db: PgPool,
}

// ---------- ошибки ----------
#[derive(Debug, thiserror::Error)]
enum AppError {
    #[error("not found")]
    NotFound,
    #[error("validation: {0}")]
    Validation(&'static str),
    #[error(transparent)]
    Db(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::Db(e) => {
                tracing::error!(error = %e, "db error");
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        let body = Json(serde_json::json!({ "error": self.to_string() }));
        (status, body).into_response()
    }
}

type AppResult<T> = Result<T, AppError>;

// ---------- хендлеры ----------
async fn list_tasks(
    State(st): State<AppState>,
    Query(p): Query<ListParams>,
) -> AppResult<Json<Vec<Task>>> {
    let tasks = sqlx::query_as!(
        Task,
        r#"SELECT id, title, done FROM tasks
           WHERE ($1::bool IS NULL OR done = $1)
           ORDER BY id LIMIT $2"#,
        p.done,
        p.limit
    )
    .fetch_all(&st.db)
    .await?;
    Ok(Json(tasks))
}

async fn create_task(
    State(st): State<AppState>,
    Json(new): Json<NewTask>,
) -> AppResult<(StatusCode, Json<Task>)> {
    if new.title.trim().is_empty() {
        return Err(AppError::Validation("title must not be empty"));
    }
    let task = sqlx::query_as!(
        Task,
        "INSERT INTO tasks (title) VALUES ($1) RETURNING id, title, done",
        new.title
    )
    .fetch_one(&st.db)
    .await?;
    Ok((StatusCode::CREATED, Json(task)))
}

async fn get_task(State(st): State<AppState>, Path(id): Path<i64>) -> AppResult<Json<Task>> {
    sqlx::query_as!(Task, "SELECT id, title, done FROM tasks WHERE id = $1", id)
        .fetch_optional(&st.db)
        .await?
        .map(Json)
        .ok_or(AppError::NotFound)
}

async fn update_task(
    State(st): State<AppState>,
    Path(id): Path<i64>,
    Json(upd): Json<UpdateTask>,
) -> AppResult<Json<Task>> {
    sqlx::query_as!(
        Task,
        r#"UPDATE tasks
           SET title = COALESCE($2, title),
               done  = COALESCE($3, done)
           WHERE id = $1
           RETURNING id, title, done"#,
        id,
        upd.title,
        upd.done
    )
    .fetch_optional(&st.db)
    .await?
        .map(Json)
        .ok_or(AppError::NotFound)
}

async fn delete_task(State(st): State<AppState>, Path(id): Path<i64>) -> AppResult<StatusCode> {
    let res = sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
        .execute(&st.db)
        .await?;
    if res.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

async fn health(State(st): State<AppState>) -> AppResult<&'static str> {
    sqlx::query("SELECT 1").execute(&st.db).await?;
    Ok("ok")
}

fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/tasks", get(list_tasks).post(create_task))
        .route(
            "/tasks/{id}",
            get(get_task).patch(update_task).delete(delete_task),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("tasks_api=debug,tower_http=debug")
        .init();

    // let url = std::env::var("DATABASE_URL")?;
    let url = "postgres://postgres:postgres@localhost:5432/shop";
    let db = PgPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await?;
    sqlx::migrate!().run(&db).await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app(AppState { db })).await?;
    Ok(())
}
