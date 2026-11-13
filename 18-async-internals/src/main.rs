use std::path::{self, Path};

use tokio::{fs::File, io::{AsyncWriteExt, BufWriter}};

const TARGET_SIZE: usize = 100 * 1024 * 1024; // 100 Mbytes
const BUFFER_SIZE: usize = 1024 * 1024; // 1 Mbytes

async fn generate_large_file(filename: &Path) -> std::io::Result<()> {
    let file = File::create(filename).await?;
    let mut writer = BufWriter::new(file);

    // Создаём повторяющейся паттерн
    let pattern = b"0123456789";
    let pattern_len = pattern.len();

    let mut written = 0;

    while written < TARGET_SIZE {
        let to_write = (TARGET_SIZE - written).min(BUFFER_SIZE);
        let mut buffer = Vec::with_capacity(to_write);

        while buffer.len() < to_write {
            let chunk_start = buffer.len() % pattern_len;
            let chunk_end = (chunk_start + (to_write - buffer.len())).min(pattern_len);
            buffer.extend_from_slice(&pattern[chunk_start..chunk_end]);
        }

        writer.write_all(&buffer).await?;
        written += to_write;
    }

    writer.flush().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let test_files = [
        "file1.txt",
        "file2.txt",
        "file3.txt",
    ];

    let mut join_handlers = vec![];

    for filename in test_files {
        let file_path = Path::new(filename);

        if !file_path.exists() {
            join_handlers.push(tokio::spawn(generate_large_file(file_path)));
        }
    }

    // Запуск одновременной генерации трёх файлов
    for handler in join_handlers {
        let _ = handler.await.expect("File create fault");
    }
}
