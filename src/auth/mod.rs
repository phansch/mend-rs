use std::{
    str::FromStr,
    env,
};
use openssl::rsa::{Rsa};
use jsonwebtoken::*;
use serde_derive::{Serialize, Deserialize};

/// JWT Claims
///
/// See https://tools.ietf.org/html/rfc7519#section-4
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    /// Issued At
    ///
    /// See https://tools.ietf.org/html/rfc7519#section-4.1.6
    iat: i64,
    /// Expiration Time
    ///
    /// See https://tools.ietf.org/html/rfc7519#section-4.1.4
    exp: i64,
    /// Issuer (In our case that's the GitHub App ID)
    ///
    /// See https://tools.ietf.org/html/rfc7519#section-4.1.1
    iss: i64,
}

/// Generates an expiring JWT token from the private key
///
/// Translation of the Ruby code at:
/// https://developer.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-a-github-app
pub fn gen_jwt(pkey: &[u8], issue_time: i64) -> String {
    let private_key = Rsa::private_key_from_pem(pkey)
        .expect("Invalid RSA Private Key");

    let app_id = i64::from_str(
        &env::var("GH_APP_ID").unwrap()
    ).expect("Could not parse GH_APP_ID to i64");

    let mut header = Header::default();
    header.alg = Algorithm::RS256;

    let claims = Claims {
        iat: issue_time,
        exp: issue_time + 600,
        iss: app_id,
    };

    let der_private_key = private_key.private_key_to_der()
        .expect("Could not build DER string");
    jsonwebtoken::encode(&header, &claims, &der_private_key)
        .expect("Could not encode JWT")
}

#[test]
fn test_gen_jwt() {
    // TODO: check if this private key has been revoked
    let test_pkey = include_bytes!("../../tests/auxiliary/test_private_key");
    env::set_var("GH_APP_ID", "1");
    let expected_token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9.eyJpYXQiOjEsImV4cCI6NjAxLCJpc3MiOjF9.R26cEMaknsi-Gdg2xdzFhW7HOC5XfK-stG49vRpAHbqu_gF56KEbcOGlWs6MiTM6Njo1vjC61ByRn6uVcsnoKLa871sqgEocZaRzmAD1Ca1GOexx97uK1xYyBdPHOUP-DygJ9t9fGXnur7TQiRwoFf8Eo9P8OFXK-BrRwVBGQwduLejj4pq2TjH6oYhfTfcImBsR8IyKIge8bwJ91pd0rjONFdxUc0ji8JI3Psm0cWo-VhzHxYC6HlfBqSfvz97qK8EYIOZjXDe6GGZyfkDlYHwXRn82N8ZJauor8lyHrI5a1sGkeI63X0ryVSki8Csv4UVnCnqKZ0pLKQSgizhI4Q";

    assert_eq!(expected_token.to_string(), gen_jwt(test_pkey, 1));
    env::remove_var("GH_APP_ID");
}
