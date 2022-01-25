use std::marker::PhantomData; 


pub struct Query<Row>(QueryPriv<Row>); 

enum QueryPriv<Row> {
    Unexecuted(QueryData<Row>), 
    Polled, 
    Finished, 
}


struct QueryData<Row> {
    row: Row, 

    columns: Vec<i32>
}