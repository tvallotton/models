




fn find_by<Model, Field, FieldType>(
    column: Field,
    value: FieldType,
) -> Result<Model, super::error::Error>
where
    Field: Find<Model, FieldType>,
{
    Field::find_by(column, value)
}

trait Find<Model, FindType = <Self as Field<Model>>::Type>
where
    Self: Field<Model>,
{
    fn find_by(column: Self, value: FindType) -> Result<Model, super::error::Error>;
}



trait Field<Model> {
    type Type;
}


impl<Model, F1, F0: Field<Model>> Field<(F0, F1)> for F0 {
    type Type = F0::Type; 
}




struct User {
    id: i32,
    email: String,
    password: String,
}

const _: () = {
    struct email;
    struct id;
    struct password;

    impl User {
        pub const email: email = email;
    }
    impl Find<User> for email {
        fn find_by(column: Self, value: Self::Type) -> Result<User, crate::orm::error::Error> {
            todo!()
        }
    }
    impl Find<User, &str> for email {
        fn find_by(column: Self, value: &str) -> Result<User, crate::orm::error::Error> {
            todo!()
        }
    }
    impl Field<User> for email {
        type Type = String;
    }
};

fn foo() {
    let x = find_by(User::email, format!("")).unwrap();
}
