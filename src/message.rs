use rocket::form::Form;
use rocket::response::Redirect;
use rocket::http::{Cookie, CookieJar};
use rocket::response::content::Html;

#[macro_export]
macro_rules! message_uri {
    ($($t:tt)*) => (rocket::uri!("/message", $crate::message:: $($t)*))
}

pub use message_uri as uri;

#[derive(FromForm)]
struct MyForm {
    message: String,
    number: isize,
}

#[post("/", data = "<form>")]
fn submit(cookies: &CookieJar<'_>, form: Form<MyForm>) -> Redirect {
    cookies.add(Cookie::new("message", form.message.to_string()));
    cookies.add(Cookie::new("number", form.number.to_string()));
    Redirect::to(uri!(index))
}

#[get("/")]
fn index(cookies: &CookieJar<'_>) -> Html<String> {
    let msg = cookies.get("message").map(|c| c.value()).unwrap_or("No message yet.");
    let num = cookies.get("number").map(|c| c.value()).unwrap_or("No number yet.");

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
            "#+&msg+r#"
            "#+&num+r#"
    
            <form action="/message" method="post" accept-charset="utf-8">
            <textarea placeholder="Your message here..."
                name="message" rows="10" cols="50"></textarea>
            <p>
                <input type="number" value="1" name="number"/></p>
            <p>
                <input type="submit" value="Set Cookie"></p>
            </form>
        
            <a href="/">Home</a>
        </body>
        </html>
    "#)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![submit, index]
}
