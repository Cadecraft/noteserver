use axum::http::header::{self, HeaderMap};

use axum_extra::extract::cookie::CookieJar;

use noteserver::auth;

pub fn get_cookie_from_jar(jar: &CookieJar, cookie_name: &str) -> Option<String> {
    let cookie_gotten = jar.get(cookie_name).cloned();
    cookie_gotten.map(|cookie| cookie.value().to_string())
}

pub fn get_token_cookie_name(dir: &str) -> String {
    format!("tok-{}", dir)
}

pub fn is_dark_theme(jar: &CookieJar) -> bool {
    let darktheme = get_cookie_from_jar(jar, "theme").unwrap_or(String::from("light"));
    darktheme == "dark"
}

pub fn valid_auth(headers: &HeaderMap) -> bool {
    match headers.get(header::AUTHORIZATION).cloned() {
        Some(val) => match val.to_str() {
            Ok(pw) => auth::is_authorized(pw),
            _ => false,
        },
        None => false,
    }
}

pub fn make_redirect_headers(to: String) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::LOCATION, to.parse().unwrap());
    headers
}
