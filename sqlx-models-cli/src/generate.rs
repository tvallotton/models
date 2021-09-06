use crate::opt::GenerateOpt;
use std::process::*;
pub async fn generate(opt: GenerateOpt) {
    let _process = tokio::process::Command::new("cargo")
        .arg("test")
        .arg("__sqlx_models_generate_migration_")
        .env("SQLX_MODELS_GENERATE_MIGRATIONS", "true")
        .env("MIGRATIONS_DIR", opt.source)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap();
}
