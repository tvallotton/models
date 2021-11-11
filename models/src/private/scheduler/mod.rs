use crate::prelude::*;
pub mod driver;
pub mod table;

use driver::*;
use table::*;
pub struct Scheduler(Mutex<Driver>);

impl Scheduler {
    pub(super) fn new() -> Self {
        Self(Mutex::new(Driver::new()))
    }

    pub fn register(&self, table: Table) {
        let is_first;
        {
            let mut driver = self.0.lock().unwrap();
            is_first = driver.is_first();
            driver.register(table)
            // release the lock
        }

        if is_first {
            std::thread::sleep(time::Duration::from_millis(250));
            self.commit()
        }
    }

    fn commit(&self) {
        let mut driver = self.0.lock().unwrap();
        driver.migrate();
        let json = driver.as_json();
        println!("<SQLX-MODELS-OUTPUT>{0}</SQLX-MODELS-OUTPUT>", json);
    }
}
