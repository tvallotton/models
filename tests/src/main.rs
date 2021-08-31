#[allow(dead_code)]
#[allow(unused_imports)]
use sqlx_models::Model;





#[derive(Model)]
struct User {
    #[primary_key(second_id)]
    id: i32,
    
    second_id: i32,


}




fn main() {}
