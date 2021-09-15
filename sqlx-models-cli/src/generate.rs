use crate::opt::GenerateOpt;
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
    fn print(self) {
        for (num, name) in self.success {
            println!(
                "Generated {}/{} {}",
                style(num).cyan(),
                style("migrate").green(),
                style(name).white()
            )
        }
    }
}

pub async fn generate(opt: GenerateOpt) {
    touch_any().await.ok();

    if !builds(&opt).await {
        println!(
            "{}: Could not compile project. No migrations were generated.",
            style("error").red()
        );
        return;
    }

    let output = tokio::process::Command::new("cargo")
        .arg("test")
        .arg("--")
        .arg("--nocapture")
        .arg("__sqlx_models_generate_migration_")
        .env("SQLX_MODELS_GENERATE_MIGRATIONS", "true")
        .env("MIGRATIONS_DIR", &opt.source)
        .output()
        .await
        .unwrap()
        .stdout;
    let output = String::from_utf8(output).unwrap();
    let regex = regex::Regex::new("<SQLX-MODELS-OUTPUT>(.+)</SQLX-MODELS-OUTPUT>").unwrap();

    let x = regex.captures(&output).unwrap();

    if let Some(json) = x.get(1) {
        from_str::<Output>(json.as_str()).unwrap().print();
    } else {
        println!("No migrations generated.")
    }

    touch_any().await.ok();
}

async fn builds(opt: &GenerateOpt) -> bool {
    tokio::process::Command::new("cargo")
        .arg("build")
        .arg("--tests")
        .env("SQLX_MODELS_GENERATE_MIGRATIONS", "true")
        .env("MIGRATIONS_DIR", &opt.source)
        // .stdout(std::process::Stdio::piped())
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
