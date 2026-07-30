mod postgresql;
mod model;

use palm_backend::model::intermediate::UserCreation;
use postgres::{Client, NoTls};
use crate::model::service::Service;

fn main() {
    //Change host and user to what they need to be
    let mut postgres_client = Client::connect("host=localhost user=postgres", NoTls).unwrap_or_else(|e| panic!("{e}: Error connecting to Postgresql database"));
    
    //Examples
    //postgres_client.create_user(&UserCreation::default());
}
