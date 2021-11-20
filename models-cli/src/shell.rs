use super::opt::ShellOpt;
use super::Result;
use sqlx::sqlite::SqliteConnectOptions;
use url::Url;

pub async fn open_shell(opt: ShellOpt) -> Result<()> {
    let database_url = Url::parse(&opt.database_url)?;

    match database_url.scheme() {
        "sqlite" => {
            let path = opt
                .database_url
                .trim_start_matches("sqlite://")
                .trim_start_matches("sqlite:");
            tokio::process::Command::new("sqlite3")
                .arg(path)
                .spawn()?
                .wait()
                .await?;
        }
        "postgres" => {
            tokio::process::Command::new("psql")
                .arg(opt.database_url)
                .spawn()?
                .wait()
                .await?;
        }
        "mysql" => {
            tokio::process::Command::new("mysql")
                .arg("-u")
                .arg(database_url.username())
                .arg("-p")
                .arg(database_url.password().unwrap_or_default())
                .arg("-h")
                .arg(database_url.domain().unwrap_or_default())
                .arg(database_url.path())
                .spawn()?
                .wait()
                .await?;
        }
        scheme => anyhow::bail!("unsupported database scheme: {:?}.", scheme),
    }

    Ok(())
}

#[test]
fn test() {
    let url = Url::parse("sqlite://database");
    println!("{:?}", url.unwrap());
}
