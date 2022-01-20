#[macro_use] extern crate rocket;

use rocket::{State, form::Form, response::{Redirect, content::Html}, fs::{FileServer, relative}};
use std::{sync::Mutex, cell::RefCell};

struct MessageState(Mutex<RefCell<String>>);

unsafe impl Send for MessageState {}
unsafe impl Sync for MessageState {}

#[derive(FromForm)]
struct MyForm {
    message: String,
}

#[post("/message", data = "<form>")]
fn post_message<'r>(form: Form<MyForm>, state: &'r State<MessageState>) -> Redirect {
    *state.inner().0.lock().unwrap().borrow_mut() = form.message.to_string();
    Redirect::to(uri!(get_message))
}

#[get("/message")]
fn get_message<'r>(state: &'r State<MessageState>) -> Html<String> {
    let msg = &*state.inner().0.lock().unwrap();
    let msg: &str = &*msg.borrow();

    Html(String::new() + r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8" />
            <meta name="viewport" content="width=device-width" />
            <title>Rocket: Cookie Message</title>
        </head>
        <body>
            <h1>Rocket Cookie Message</h1>
            "#+msg+r#"
    
            <form action="/message" method="post" accept-charset="utf-8">
            <textarea placeholder="Your message here..."
                name="message" rows="10" cols="50"></textarea>
            <p>
                <input type="submit" value="Set Message"></p>
            </form>
        
            <a href="/">Home</a>
        </body>
        </html>
    "#)
}

#[get("/language/<language>")]
fn language_selection(language: &str) -> Html<String> {
    Html(String::new() + r#"
        <h1> You selected the region "#+language+r#"</h1>
        <p>
        <a href="/"> <h1> back </h1> </a> </p>
    "#)
}


#[get("/")]
fn index() -> Html<String> {
    let languages = ["Bulgaria", "France", "Hungary", "Romania", "Turkey", "Croatia", "Germany", "Netherlands", "Russia", "CzechRepublic", "GreatBritain", "Poland","Serbia"];
    Html(String::new() + r#"
        <!DOCTYPE html>
        <html>
        <head>
            <meta charset="utf-8" />
            <meta name="viewport" content="width=device-width" />
            <title>Rocket</title>

            <style>
                .flex-container {
                    display: flex;.png
                    flex-direction: row;
                    flex-wrap: wrap;
                    justify-content: center;
                    gap: 20px;
                    height: 300;
                }
            </style>
        </head>
        <body style="background-color: #EEEEEE;">
            <div class="flex-container">
                "# 
                    + &languages
                        .map(|lang| String::new() + r#"
                            <a href=/language/"#+lang+r#">  <img src="/assets/img/lang/"#+lang+r#".png" alt="language selection" width="150" height="100"> </a>  
                        "#)
                        .join("\n") +
                r#"
            </div>
        </body>
        </html>
    "#)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, language_selection])
        .mount("/assets", FileServer::from(relative!("assets")))
        .manage(MessageState(Mutex::new(RefCell::new("No message".to_string()))))
}
