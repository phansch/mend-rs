# mend-rs

[![Build Status](https://travis-ci.com/phansch/mend-rs.svg?branch=master)](https://travis-ci.com/phansch/mend-rs)

## Development

Run `bin/setup` to install all dependencies, including docker and postgresql.

### Postgresql

#### Open a console

    sudo -u postgres psql

### Running the docker container

```shell
docker run -t -p 4000:4000 mend
```

## Deployment

```shell
git push dokku
```

## Dokku/Server stuff

```shell
dokku logs mend-rs -t
```

Need to also set the following ENV variables on the server:

* GH_CLIENT_ID
* GH_CLIENT_SECRET
* DATABASE_URL

You can find them at the bottom of https://github.com/settings/apps/mend-rs

## User Workflow

1. Users sign up with GitHub
2. They see a repo selection screen
3. They select a repo to register PR hook
4. Done, they will see the comments in new PRs

It should only comment on changed or added lines

## Technical Workflow

1. [ ] Register PR hook
1. [ ] On PR, we will receive the hook notification, and start a new docker container with nightly rust + clippy
1. [ ] Run `RUSTFLAGS="-Z unstable-options --error-format=json" cargo +nightly clippy` in container
1. [ ] Somehow get the json back? Maybe via web api? Or maybe through Filesystem?

Check: https://firejail.wordpress.com/
Check clippy-service for dokku/travis deploy setup

## Snippets

```rust
// Rate limit status:
let status = core.run(github.rate_limit().get()).unwrap();
println!("{:#?}", status);

// Stuff:
let f = github.repo("phansch", "rust-clippy").pulls().get(1).review_comments().create(&opts);
let f = github.repo("phansch", "rust-clippy").pulls().get(1).get();
match core.run(f) {
    Ok(pr) => println!("{:?}", pr.patch_url),
    Err(err) => println!("err {}", err),
}

// ReviewCommentOptions:
let opts = ReviewCommentOptions {
    body: "abc".to_string(),
    commit_id: "62733ca4cc6e9716b566a809dc3dd88f8b94690e".to_string(),
    path: ".travis.yml".to_string(),
    position: 5
};
 ```
