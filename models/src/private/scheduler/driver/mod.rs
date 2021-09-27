use crate::prelude::*;
pub(crate) mod actions;
pub mod migration;
mod queue;
mod report;
mod schema;
use actions::Actions;
use queue::*;
pub(crate) use report::*;
use schema::*;

pub(crate) struct Driver {
    result: Result<Schema>,
    queue: Queue,
    success: Vec<Report>,
}

impl Driver {
    pub fn new() -> Self {
        let result = Schema::new();
        Self {
            result,
            queue: Queue::new(),
            success: vec![],
        }
    }
    pub fn is_first(&self) -> bool {
        self.queue.len() == 0
    }

    pub fn register(&mut self, table: Table) {
        self.queue.insert(table)
    }
    pub fn as_json(&self) -> String {
        let error = if let Err(err) = &self.result {
            err.as_json()
        } else {
            "null".into()
        };
        format!(
            r#"{{"success": {success:?},"error": {error}}}"#,
            success = &self.success,
            error = error
        )
    }

    pub fn migrate(&mut self) {
        loop {
            match self.queue.pop() {
                Some(target) => self.migrate_table(target),

                None => {
                    if self.queue.len() != 0 && self.result.is_ok() {
                        self.result = Err(Error::Cycle(self.queue.remaining_tables()));
                    }
                    break;
                }
            }
        }
    }

    pub fn migrate_table(&mut self, target: Table) {
        if let Err(error) = self.try_migration(target) {
            self.result = Err(error);
        }
    }

    fn try_migration(&mut self, target: Table) -> Result {
        let migrations = self.get_migrations(target)?;
        for mig in migrations {
            
            if let Some(report) = mig.commit()? {
                self.success.push(report);
            }
        }
        Ok(())
    }

    fn get_migrations(&mut self, target: Table) -> Result<Vec<Migration>> {
        println!("get_migrations");
        let schema = self.result.as_mut().map_err(|x| x.clone())?;
        let actions = Actions::new(&schema, &target)?;
        
        let mut migrations = actions.as_migrations()?;
        
        for migr in &mut migrations {
        
            let old_schema = schema.clone();
            for stmt in migr.up() {
                schema.update(&stmt)?;
            }
            migr.create_down(old_schema, schema, &target.name)?;
            
        }
        Ok(migrations)
    }
}
