use anyhow::{
    Error,
    Result,
};
use console::style;
use serde::*;
use serde_json::from_str;

use super::opt::GenerateOpt;

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
    fn print(self, source: &str) -> Result<()> {
        for (num, name) in self.success {
            println!(
                "{}: {}/{}{}{}",
                style("Generated").bold().green(),
                style(source),
                style(num).cyan(),
                style("_").dim(),
                style(name)
            )
        }

        if let Some(err) = self.error {
            Err(Error::msg(err.message))
        } else {
            Ok(())
        }
    }
}

pub async fn generate(opt: GenerateOpt) -> Result<()> {
    use anyhow::*;
    std::fs::create_dir_all(&opt.source).context("Unable to create migrations directory")?;
    opt.validate().await?;
    touch_any().await.ok();

    if !builds(&opt.database_url, &opt.source).await {
        return Err(Error::msg(
            "could not compile project. No migrations were generated.",
        ));
    }
    let filter_tests = format!(
        "__models_generate_migration_{}",
        opt.table.as_deref().unwrap_or("")
    );
    let output = tokio::process::Command::new("cargo")
        .arg("test")
        .arg("--")
        .arg("--nocapture")
        .arg(&filter_tests)
        .env("MODELS_GENERATE_MIGRATIONS", "true")
        .env("MIGRATIONS_DIR", &opt.source)
        .env("DATABASE_URL", &opt.database_url)
        .env("MODELS_GENERATE_DOWN", opt.reversible.to_string())
        .output()
        .await
        .unwrap()
        .stdout;
    let output = String::from_utf8(output).unwrap();
    let regex = regex::Regex::new("<SQLX-MODELS-OUTPUT>(.+)</SQLX-MODELS-OUTPUT>").unwrap();

    if output.contains("running 0 tests") {
        if let Some(table) = &opt.table {
            println!("No models named {}.", table)
        } else {
            println!("No models in the application")
        }
        return Ok(());
    }
    let x = regex.captures(&output).expect(&output);

    if let Some(json) = x.get(1) {
        from_str::<Output>(json.as_str())
            .expect(json.as_str())
            .print(&opt.source)?;
    } else {
        println!("Everything is up to date.");
    }
    touch_any().await.ok();
    Ok(())
}

async fn builds(database_url: &str, source: &str) -> bool {
    tokio::process::Command::new("cargo")
        .arg("build")
        .arg("--tests")
        .env("MODELS_GENERATE_MIGRATIONS", "true")
        .env("MIGRATIONS_DIR", database_url)
        .env("DATABASE_URL", source)
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap()
        .success()
}

pub async fn touch_any() -> Result<()> {
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
