use crate::prelude::*;
pub mod driver;
pub mod table;

use driver::*;
use table::*;
pub struct Scheduler(Result<Mutex<Driver>>);

impl Scheduler {
    pub(super) fn new() -> Self {
        Self(Driver::new().map(|op| op.into()))
    }

    pub fn register(&self, table: Table) {
        pretty_env_logger::try_init().ok();

        match &self.0 {
            | Ok(mutex) => {
                let mut driver = mutex.lock().unwrap();
                driver.register(table);
                if driver.is_first() {
                    std::thread::sleep(time::Duration::from_millis(250));
                    self.commit()
                }
            }
            | Err(error) => error.log(),
        }
    }

    fn migrate(&self) -> Result<Vec<Report>> {
        self.0
            .as_ref()
            .map_err(Clone::clone)?
            .lock()
            .unwrap()
            .migrate()
    }

    fn commit(&self) {
        match self.migrate() {
            | Ok(reports) => {
                print!(
                    r#"<SQLX-MODELS-OUTPUT>{{"success": {reports:?},"error": null}}</SQLX-MODELS-OUTPUT>"#,
                    reports = reports,
                )
            }
            | Err(error) => error.log(),
        }
    }
}
