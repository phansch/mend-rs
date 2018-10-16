#[cfg(test)]
mod tests {
    use mend_rs::mend_github::*;
    use mockito::mock;
    use mockito::SERVER_URL;

    const GH_URL: &'static str = SERVER_URL;

    #[test]
    fn test_exchange_code_for_access_token_ok() {
        let _m = mock("POST", "/login/oauth/access_token")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("access_token=the_access_token&token_type=bearer")
            .create();

        let access_token = MendGithub::exchange_code_for_access_token(
            "client-id",
            "client-secret",
            "somecode",
            "redirect-uri",
            "state",
            GH_URL,
        );
        assert_eq!("the_access_token", access_token);
    }

    #[test]
    #[should_panic(expected = "Incorrect redirect_uri")]
    fn test_exchange_code_for_access_token_incorrect_redirect_uri() {
        let _m = mock("POST", "/login/oauth/access_token")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("error=redirect_uri_mismatch")
            .create();

        let access_token = MendGithub::exchange_code_for_access_token(
            "client-id",
            "client-secret",
            "somecode",
            "ftp://example.com/fail",
            "state",
            GH_URL,
        );
        assert_eq!("the_access_token", access_token);
    }
}
