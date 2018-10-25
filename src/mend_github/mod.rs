use std::{
    fs::{create_dir_all, File},
    io::{Cursor, Read, Write},
    path::PathBuf,
};

use ::{
    hubcaps::*,
    tempfile::TempDir,
    tokio_core::reactor::Core,
    futures::Stream,
    log::*,
};

use crate::{
    review_comment::ReviewComment,
};

pub struct MendGithub {
    core: Core,
    token: String,
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
}
