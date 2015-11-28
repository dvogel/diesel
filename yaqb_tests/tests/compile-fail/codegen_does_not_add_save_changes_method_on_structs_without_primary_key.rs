#![feature(custom_derive, plugin, custom_attribute)]
#![plugin(yaqb_codegen)]
#[macro_use]
extern crate yaqb;

use yaqb::*;

table! {
    users {
        id -> Serial,
        name -> VarChar,
        hair_color -> VarChar,
    }
}

#[derive(Queriable)]
#[changeset_for(users)]
pub struct User {
    name: String,
    hair_color: String,
}

fn main() {
    let connection = Connection::establish("").unwrap();
    let mut user = User {
        name: "Sean".to_string(),
        hair_color: "black".to_string(),
    };
    user.save_changes(&connection);
    //~^ ERROR no method named `save_changes` found
}
