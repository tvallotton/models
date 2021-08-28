


#[derive(Model)]
struct User {
    #[primary_key]
    id: i32,
    #[unique]
    email: String,
    password: String,
}

impl Model for User {
    fn table(dialect: &dyn Dialect) -> Table {
        Table {
            name: "User",
            if_not_exists: false,
            or_replace: false,
            columns:  vec![
                Column {
                    name: "id",
                    r#type: i32::to_sql(),
                    constraints: vec![Constraint::Unique {
                        columns: vec!["id"],
                        is_primary_key=true
                    }]
                }
            ],
           constraints: Vec<TableConstraint>

        }
    }
}
