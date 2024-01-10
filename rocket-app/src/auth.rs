use rocket::request::{FromRequest, Request, Outcome};
use rocket::http::Status;
use base64::{Engine as _, engine::general_purpose};

pub struct BasicAuth {
    pub username: String,
    pub password: String
}

impl BasicAuth {

    fn form_authorization_header(header: &str) -> Option<BasicAuth> {
        let split: Vec<_> = header.split_whitespace().collect();
        if split.len() != 2 { return None; }

        if let Some(basic) = split.get(0) {
            if *basic != "Basic" { return None; }
        }

        Self::from_base64_encoded(split[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth> {

        let decoded = general_purpose::STANDARD.decode(base64_string).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let split: Vec<_> = decoded_str.split(":").collect();

        if split.len() != 2 { return None; }

        let username = (*split.get(0)?).to_string();
        let password = (*split.get(1)?).to_string();

        Some(BasicAuth { 
            username, 
            password
        })
    }

}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        if let Some(auth_header) = auth_header { 
            if let Some(auth) = Self::form_authorization_header(auth_header) {
                return Outcome::Success(auth) 
            }
        }
        Outcome::Error((Status::Unauthorized, ()))
    }
}