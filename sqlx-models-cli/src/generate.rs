use console::style;
use serde::*;
use serde_json::from_str;
#[derive(Serialize, Deserialize)]

struct MigrationError {
    kind: String,
    message: String,
}
#[derive(Serialize, Deserialize)]
struct Output {
    success: Vec<(i64, String)>,
    error: Option<MigrationError>,
}

impl Output {
    fn print(self, source: &str) {
        for (num, name) in self.success {
            println!(
                "{}: {}/{} {}",
                style("Generated").bold().green(),
                style(source),
                style(num).cyan(),
                style(name)
            )
        }

        if let Some(err) = self.error {
            println!("{}: {}", style(err.kind).red().bold(), err.message)
        }
    }
}

pub async fn generate(database_url: &str, table: Option<String>, source: &str) {
    touch_any().await.ok();

    if !builds(database_url, source).await {
        println!(
            "{}: Could not compile project. No migrations were generated.",
            style("error").red()
        );
        return;
    }
    let filter_tests = format!(
        "__sqlx_models_generate_migration_{}",
        table.as_deref().unwrap_or("")
    );
    let output = tokio::process::Command::new("cargo")
        .arg("test")
        .arg("--")
        .arg("--nocapture")
        .arg(&filter_tests)
        .env("SQLX_MODELS_GENERATE_MIGRATIONS", "true")
        .env("MIGRATIONS_DIR", source)
        .env("DATABASE_URL", database_url)
        .output()
        .await
        .unwrap()
        .stdout;
    let output = String::from_utf8(output).unwrap();
    let regex = regex::Regex::new("<SQLX-MODELS-OUTPUT>(.+)</SQLX-MODELS-OUTPUT>").unwrap();

    if output.contains("running 0 tests") {
        if let Some(table) = table {
            println!("No models named {}.", &table)
        } else {
            println!("No models in the application")
        }
        return;
    }
    let x = regex.captures(&output).expect(&output);

    if let Some(json) = x.get(1) {
        from_str::<Output>(json.as_str()).unwrap().print(source);
    } else {
        println!("Everything is up to date.");
    }
    touch_any().await.ok();
}

async fn builds(database_url: &str, source: &str) -> bool {
    tokio::process::Command::new("cargo")
        .arg("build")
        .arg("--tests")
        .env("SQLX_MODELS_GENERATE_MIGRATIONS", "true")
        .env("MIGRATIONS_DIR", database_url)
        .env("DATABASE_URL", source)
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap()
        .success()
}

use std::error::Error;

pub async fn touch_any() -> Result<(), Box<dyn Error>> {
    let mut listdir = tokio::fs::read_dir("src/").await?;
    while let Some(entry) = listdir.next_entry().await? {
        let file_name = entry.file_name();
        let regex = regex::Regex::new(r".+\.rs")?;
        if regex.is_match(file_name.to_str().unwrap()) {
            // println!("{}", format!("src/{}", file_name.to_str().unwrap()));
            let success = tokio::process::Command::new("touch")
                .arg(&format!("src/{}", file_name.to_str().unwrap()))
                .spawn()?
                .wait()
                .await?
                .success();
            if success {
                break;
            }
        }
    }
    Ok(())
}
