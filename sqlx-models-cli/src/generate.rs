use crate::opt::GenerateOpt;

pub async fn generate(opt: GenerateOpt) {
    touch_any().await.ok();
//  let _process = tokio::process::Command::new("cargo")
//         .arg("build")
//         .arg("__sqlx_models_generate_migration_")
//         .env("SQLX_MODELS_GENERATE_MIGRATIONS", "true")
//         .env("MIGRATIONS_DIR", opt.source)
//         .stdout(std::process::Stdio::inherit())
//         .spawn()
//         .unwrap()
//         .wait()
//         .await
//         .unwrap();
    let _process = tokio::process::Command::new("cargo")
        .arg("test")
        .arg("--")
        .arg("--nocapture")
        .arg("__sqlx_models_generate_migration_")
        .env("SQLX_MODELS_GENERATE_MIGRATIONS", "true")
        .env("MIGRATIONS_DIR", opt.source)
        .stdout(std::process::Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap();

    touch_any().await.ok();
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
