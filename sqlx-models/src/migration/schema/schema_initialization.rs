use super::*;
use crate::prelude::*; 
impl Schema {
    /// constructs a new Schema from the "migrations/" directory.
    pub fn new(dialect: Dialect) -> Self {
        let mut out = Self {
            dialect,
            tables: HashMap::new(),
        };
        out.init();
        out
    }

    /// Computes the current state of the schema
    /// from the "migrations/" directory.
    fn init(&mut self) {
        let stmts = self.get_statements();
        for stmt in stmts {
            self.update_schema(stmt);
        }
    }
    /// It retrieves a vec of all statements in the "migrations/" directory
    /// In the order they were written.
    fn get_statements(&mut self) -> Vec<Statement> {
        self.read_dir()
            .into_iter()
            .filter(|file| file.is_file())
            .map(read_to_string)
            .map(Result::unwrap)
            .map(|x| x.to_lowercase())
            .map(|sql| parse_sql(&self.dialect, &sql))
            .map(Result::unwrap)
            .fold(vec![], |mut a, mut b| {
                a.append(&mut b);
                a
            })
    }
    /// returns a list of all the files in the migrations directory.
    fn read_dir(&self) -> Vec<PathBuf> {
        let mut dir: Vec<_> = read_dir(&*MIGRATIONS_DIR)
            .or_else(|_| {
                create_dir(&*MIGRATIONS_DIR) //
                    .and_then(|_| read_dir(&*MIGRATIONS_DIR))
            })
            .expect(&format!("Could not read the \"{}\" directiory.", *MIGRATIONS_DIR))
            .map(|x| x.unwrap().path())
            .collect();
        dir.sort();
        dir
    }
}
