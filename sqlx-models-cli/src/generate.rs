use crate::opt::GenerateOpt;

pub async fn generate(opt: GenerateOpt) {
    let _process = tokio::process::Command::new("cargo")
        .arg("test")
        .arg("__sqlx_models_generate_migration_")
        .env("SQLX_MODELS_GENERATE_MIGRATIONS", "true")
        .env("MIGRATIONS_DIR", opt.source)
        .spawn()
        .unwrap()
        .wait()
        .await
        .unwrap();
}
