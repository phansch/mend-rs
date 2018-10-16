use std::{
    collections::HashMap,
    fs::{create_dir_all, File},
    io::{Cursor, Read, Write},
    path::PathBuf,
    result,
};

use ::{
    hubcaps::*,
    tempfile::TempDir,
    tokio_core::reactor::Core,
    url::*,
    futures::Stream,
    log::*,
    reqwest::header::{Headers, Authorization},
    serde_derive::Deserialize,
};

use crate::{
    review_comment::ReviewComment,
};

pub struct MendGithub {
    core: Core,
    token: String,
}

#[derive(Deserialize)]
pub struct InstallationAccessToken {
    pub token: String,
    pub expires_at: String,
}

static USERAGENT: &'static str = "mend.rs - Rust Dev Tools as a service (phansch.net)";

impl MendGithub {
    pub fn new_from_token(token: &str) -> MendGithub {
        let core = Core::new().expect("reactor fail");
        MendGithub {
            core,
            token: token.to_string(),
        }
    }


    /// Authenticate as an installation
    ///
    /// See https://developer.github.com/apps/building-github-apps/authenticating-with-github-apps/#authenticating-as-an-installation
    pub fn authenticate_installation(installation_id: i32, jwt: &str) -> InstallationAccessToken {
        let url = format!(
            "https://api.github.com/apps/installations/{}/access_tokens",
            installation_id
        );
        let mut headers = Headers::new();
        headers.set(Authorization(jwt.to_string()));
        let client = reqwest::Client::new();

        client.post(&url).send().unwrap().json().unwrap()
    }

    pub fn userinfo(&mut self) -> hubcaps::users::AuthenticatedUser {
        let github = Github::new(
            USERAGENT.to_string(),
            Credentials::Token(self.token.to_string()),
        );
        let f = github.users().authenticated();
        self.core
            .run(f)
            .expect("Could not get information of authenticated user")
    }

    pub fn repos(&mut self, username: &str) -> Vec<hubcaps::repositories::Repo> {
        let github = Github::new(
            USERAGENT.to_string(),
            Credentials::Token(self.token.to_string()),
        );
        let f = github.user_repos(username).iter(&Default::default()).collect();
        self.core
            .run(f)
            .expect(&format!("Could not fetch the repos of user {}", username))
    }

    pub fn download_archive(&mut self, user: &str, repo: &str, git_ref: &str) -> PathBuf {
        let tmp_dir = TempDir::new().expect("Could not create tempdir");
        let url = format!(
            "https://codeload.github.com/{user}/{repo}/zip/{git_ref}",
            user = user,
            repo = repo,
            git_ref = git_ref
        );
        let mut resp = reqwest::get(&url).unwrap();
        assert!(resp.status().is_success());

        let mut buf: Vec<u8> = vec![];
        resp.copy_to(&mut buf).unwrap();

        let reader = Cursor::new(buf);

        let mut archive =
            zip::ZipArchive::new(reader).expect("Could not create ZipArchive from response body");
        let mut paths: Vec<String> = Vec::new();
        // for every file, ZipArchive identified in the response,
        // we try to unpack it into the specified `tmp_dir`
        for i in 0..archive.len() {
            let mut zip_file = archive.by_index(i).unwrap();
            let extracted_path = tmp_dir.path().join(zip_file.name());
            let full_path = extracted_path.as_path();

            if zip_file.size() == 0 {
                create_dir_all(full_path).unwrap();
            } else {
                let mut writer = File::create(full_path).unwrap();
                let mut buffer: Vec<u8> = vec![];
                zip_file.read_to_end(&mut buffer).unwrap();
                writer.write_all(&buffer).unwrap();

                paths.push(full_path.to_string_lossy().into_owned());
            }
        }

        tmp_dir.into_path().join(format!("{}-{}", repo, git_ref))
    }

    pub fn get_diff_url(&mut self, user: &str, repo: &str, pr_id: u64) -> String {
        let github = Github::new(
            USERAGENT.to_string(),
            Credentials::Token(self.token.to_string()),
        );
        let f = github.repo(user, repo).pulls().get(pr_id).get();
        self.core
            .run(f)
            .expect(&format!("Could not fetch PR {}/{}", user, repo))
            .diff_url
    }

    pub fn post_comments(&mut self, comments: &[ReviewComment], user: &str, repo: &str) {
        let github = Github::new(
            USERAGENT.to_string(),
            Credentials::Token(self.token.to_string()),
        );
        for comment in comments {
            let opts = review_comments::ReviewCommentOptions {
                body: comment.body.to_string(),
                commit_id: "62733ca4cc6e9716b566a809dc3dd88f8b94690e".to_string(),
                path: comment.path.to_string(),
                position: comment.position,
            };
            let f = github
                .repo(user, repo)
                .pulls()
                .get(10)
                .review_comments()
                .create(&opts);
            match self.core.run(f) {
                Ok(comment) => info!("comment: {:?}", comment),
                Err(err) => error!("Error when posting comments {:?}", err),
            }
        }
    }

    /// Exchanges the given `code` for an `access_token`
    ///
    /// This is [Step 2][step_2] of the GitHub auth process.
    /// The `code` is obtained from Step 1.
    ///
    /// [step_2]: https://developer.github.com/apps/building-github-apps/identifying-and-authorizing-users-for-github-apps/#2-users-are-redirected-back-to-your-site-by-github
    pub fn exchange_code_for_access_token(
        client_id: &str,
        client_secret: &str,
        code: &str,
        redirect_uri: &str,
        state: &str,
        gh_url: &str,
    ) -> String {
        let url = &format!("{}/login/oauth/access_token", gh_url);
        let mut params = HashMap::new();
        params.insert("client_id", client_id);
        params.insert("client_secret", client_secret);
        params.insert("code", code);
        params.insert("redirect_uri", redirect_uri);
        params.insert("state", state);

        let client = reqwest::Client::new();
        let mut res = client.post(url).form(&params).send().unwrap();
        parse_access_token_response(&res.text().unwrap()).unwrap()
    }
}

fn parse_access_token_response(response_text: &str) -> result::Result<String, String> {
    let url =
        Url::parse(&format!("http://someurl.com?{}", response_text)).expect("Could not parse URL");
    let mut response_query_pairs = url.query_pairs();
    let first_pair = response_query_pairs.next().unwrap();
    if first_pair.0 == "error" {
        if first_pair.1 == "redirect_uri_mismatch" {
            Err("Incorrect redirect_uri".to_string())
        } else {
            Err(format!("Something else went wrong: {}", response_text))
        }
    } else {
        Ok(first_pair.1.to_string())
    }
}

#[test]
fn test_parse_access_token_response_error() {
    let response = "error=redirect_uri_mismatch&error_description=The+redirect_uri+MUST+match+the+registered+callback+URL+for+this+application.&error_uri=https%3A%2F%2Fdeveloper.
github.com%2Fapps%2Fmanaging-oauth-apps%2Ftroubleshooting-authorization-request-errors%2F%23redirect-uri-mismatch2";
    let result = parse_access_token_response(response);

    assert_eq!(Err("Incorrect redirect_uri".to_string()), result);
}

#[test]
fn test_parse_access_token_response_other_error() {
    let response = "error=some_other_error&error_description=The+redirect_uri+MUST+match+the+registered+callback+URL+for+this+application.&error_uri=https%3A%2F%2Fdeveloper.
github.com%2Fapps%2Fmanaging-oauth-apps%2Ftroubleshooting-authorization-request-errors%2F%23redirect-uri-mismatch2";
    let result = parse_access_token_response(response);

    assert_eq!(
        Err(format!("Something else went wrong: {}", response)),
        result
    );
}
