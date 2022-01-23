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

pub struct SQLFile {
    description: String,
    content: String,
}

impl SQLFile {
    fn commit(self) -> Result<()> {
        todo!()
    }
}

#[derive(Default)]
pub struct SQLFiles(pub Vec<SQLFile>);

impl SQLFiles {
    fn commit(self) -> Result<()> {
        for file in self.0 {
            file.commit()?;
        }
        Ok(())
    }
}

pub(crate) struct Driver {
    pub(crate) schema: Schema,
    pub(crate) queue: Queue,
    pub(crate) success: Vec<Report>,
}

impl Driver {
    pub fn new() -> Result<Self> {
        let schema = Schema::new()?;
        Ok(Self::from_schema(schema))
    }

    #[cfg(test)]
    pub fn _from_sql(sql: &str) -> Self {
        let schema = Schema::_from_sql(sql).unwrap();
        Self::from_schema(schema)
    }

    pub fn migrate2(&mut self) -> Result<Vec<SQLFile>> {
        todo!()
    }

    pub fn from_schema(schema: Schema) -> Self {
        Self {
            schema,
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

    /// This function will remove one table from the
    /// queue and will attempt to generate the
    /// appropriate migrations for it until there
    /// are no tables left to remove.
    pub fn migrate(&mut self) -> Result<Vec<Report>> {
        // It first removes dependencies to tables which where not registered
        // so foreign keys can be used on structs not defined by the user.
        self.queue.remove_unregistered();
        loop {
            match self.queue.pop() {
                | Some(target) => self.migrate_table(target)?,
                | None => {
                    if self.queue.len() != 0 {
                        break Err(Error::Cycle(self.queue.remaining_tables()));
                    } else {
                        break Ok(std::mem::take(&mut self.success));
                    }
                }
            }
        }
    }

    fn migrate_table(&mut self, target: Table) -> Result {
        let migrations = self.get_migrations(target)?;
        for mig in migrations {
            if let Some(report) = mig.commit()? {
                self.success.push(report);
            }
        }
        Ok(())
    }

    fn get_migrations(&mut self, target: Table) -> Result<Vec<Migration>, Error> {
        let schema = &mut self.schema;
        let actions = Actions::new(schema, &target)?;
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
