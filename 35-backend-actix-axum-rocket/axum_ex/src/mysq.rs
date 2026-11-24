use crate::{BoardData, CustomResult, TaskData};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{MySql, Pool, Row};

#[derive(Clone)]
pub struct Boards {
    pool: Pool<MySql>,
}

impl Boards {
    pub async fn new() -> Self {
        let conn_str = std::env::var("DATABASE_URL").unwrap();
        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .connect(&conn_str)
            .await
            .unwrap();
        Self { pool }
    }

    pub async fn create_board(&self, data: &BoardData) -> CustomResult<BoardData> {
        let id = sqlx::query("INSERT INTO boards (name) VALUES(?)")
            .bind(&data.name)
            .execute(&self.pool)
            .await?
            .last_insert_id();
        self.read_board(id).await
    }

    pub async fn read_boards(&self) -> CustomResult<Vec<BoardData>> {
        let rows = sqlx::query("SELECT id, name FROM boards")
            .fetch_all(&self.pool)
            .await?;
        rows.into_iter()
            .map(|row| {
                let id: u64 = row.try_get("id")?;
                Ok(BoardData {
                    id: Some(id.to_string()),
                    name: row.try_get("name")?,
                    tasks: Vec::new(),
                })
            })
            .collect()
    }

    pub async fn read_board(&self, id: u64) -> CustomResult<BoardData> {
        let row = sqlx::query("SELECT name FROM boards WHERE id=?")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;
        Ok(BoardData {
            id: Some(id.to_string()),
            name: row.try_get("name")?,
            tasks: Vec::new(),
        })
    }

    pub async fn delete_board(&self, id: u64) -> CustomResult<BoardData> {
        let board = self.read_board(id).await?;
        sqlx::query("DELETE FROM boards WHERE id=?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(board)
    }

    pub async fn update_board(&self, id: u64, data: &BoardData) -> CustomResult<BoardData> {
        sqlx::query("UPDATE boards SET name=? WHERE id=?")
            .bind(&data.name)
            .bind(id)
            .execute(&self.pool)
            .await?;
        self.read_board(id).await
    }

    pub async fn create_task(&self, id: u64, data: &TaskData) -> CustomResult<TaskData> {
        sqlx::query("INSERT INTO tasks (board_id, name, description) VALUES(?, ?, ?)")
            .bind(id)
            .bind(&data.name)
            .bind(&data.description)
            .execute(&self.pool)
            .await?;
        self.read_task(id, &data.name).await
    }

    pub async fn read_tasks(&self, id: u64) -> CustomResult<Vec<TaskData>> {
        let rows = sqlx::query("SELECT name, description FROM tasks WHERE board_id=?")
            .bind(id)
            .fetch_all(&self.pool)
            .await?;
        rows.into_iter()
            .map(|row| {
                Ok(TaskData {
                    name: row.try_get("name")?,
                    description: row.try_get("description")?,
                })
            })
            .collect()
    }

    pub async fn read_task(&self, id: u64, name: &str) -> CustomResult<TaskData> {
        let row = sqlx::query("SELECT name, description FROM tasks WHERE board_id=? AND name=?")
            .bind(id)
            .bind(name)
            .fetch_one(&self.pool)
            .await?;
        Ok(TaskData {
            name: row.try_get("name")?,
            description: row.try_get("description")?,
        })
    }

    pub async fn update_task(
        &self,
        id: u64,
        name: &str,
        data: &TaskData,
    ) -> CustomResult<TaskData> {
        sqlx::query("UPDATE tasks SET name=?, description=? WHERE board_id=? AND name=?")
            .bind(&data.name)
            .bind(&data.description)
            .bind(id)
            .bind(name)
            .execute(&self.pool)
            .await?;
        self.read_task(id, &data.name).await
    }

    pub async fn delete_task(&self, id: u64, name: &str) -> CustomResult<TaskData> {
        let task = self.read_task(id, name).await?;
        sqlx::query("DELETE FROM tasks WHERE board_id=? AND name=?")
            .bind(id)
            .bind(name)
            .execute(&self.pool)
            .await?;
        Ok(task)
    }
}
