#![feature(async_await, await_macro)]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate dotenv_codegen;

use tide::{error::ResultExt, response, Context, EndpointResult};
use http::status::StatusCode;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct User {
     name: String,
     height: u32
 }

 #[derive(Default)]
struct  Database {
     contents: Mutex<Vec<User>>,
}

impl Database {

    fn insert(&self, user: User) -> usize {
        let mut table = self.contents.lock().unwrap();
        table.push(user);
        table.len() - 1
    }

    fn get_all(&self) -> Vec<User> {
        self.contents.lock().unwrap().clone()
    }

    fn get(&self, id: usize) -> Option<User> {
        println!("id was gest");
        let table = self.contents.lock().unwrap();
        table.get(id).cloned()
    }

    fn set(&self, id: usize, user: User) -> bool {
        let mut table = self.contents.lock().unwrap();

        if let Some(old_user) = table.get_mut(id) {
            *old_user = user;
            true
        } else {
            false
        }
    }

    fn delete(&self, id: usize) -> bool {
        let mut table = self.contents.lock().unwrap();

        if let Some(_del_user) = table.get_mut(id) {
            table.remove(id);
            true
        } else {
            false
        }
    }

}

fn get_server_port() -> u32 {
    dotenv!("PORT").parse::<u32>()
    .unwrap_or(8123)
}

// Endpoint:
//      GET /
async fn handler_hello_world(_cx: Context<(Database)>) -> &'static str {
    "Hello, World!"
}

// Endpoint:
//      GET /users
async fn handler_get_users(cx: Context<(Database)>) -> EndpointResult {
    println!("get all Users");
    Ok(response::json(cx.app_data().get_all()))
}

// Endpoint:
//      GET /user/:id
async fn handler_get_user(cx: Context<Database>) -> EndpointResult {
    println!("id was get");
    let id = cx.param("id").client_err()?;
    if let Some(user) = cx.app_data().get(id) {
        Ok(response::json(user))
    } else {
        Err(StatusCode::NOT_FOUND)?
    }
}

// Endpoint:
//      POST /users
async fn handler_create_user(mut cx: Context<Database>) -> EndpointResult<String> {
        println!("User Created");
    let user = await!(cx.body_json()).client_err()?;
    Ok(cx.app_data().insert(user).to_string())
}

// Endpoint:
//      PATCH /user/:id
async fn handler_update_user(mut cx: Context<Database>) -> EndpointResult<()> {
    let user = await!(cx.body_json()).client_err()?;
    let id = cx.param("id").client_err()?;

    if cx.app_data().set(id, user) {
        Ok(())
    } else {
        Err(StatusCode::NOT_FOUND)?
    }
}

// Endpoint:
//      DELETE /user/:id
async fn handler_delete_user(cx: Context<Database>) -> EndpointResult<String> {
    let id = cx.param("id").client_err()?;
    Ok(cx.app_data().delete(id).to_string())
}

fn main() {
    let mut app = tide::App::new(Database::default());

    app.at("/").get(handler_hello_world);

    app.at("/user/:id")
        .get(handler_get_user)
        .patch(handler_update_user)
        .delete(handler_delete_user);
    app.at("/users")
        .get(handler_get_users)
        .post(handler_create_user);

    app.serve(format!("{}:{}", "0.0.0.0", get_server_port())).unwrap();
}
