use crate::backend::config;
use crate::models::User;
use maud::{html, Markup, DOCTYPE};

pub(in crate::frontend) fn signup_link() -> String {
    // let rand: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
    let rand = "fairdiceroll";
    format!(
        "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}/auth&state={}",
        config("mend_gh_client_id"),
        config("mend_hostname"),
        rand
    )
}

pub(in crate::frontend) fn header(page_title: &Option<&str>) -> Markup {
    html! {
        (DOCTYPE)
        html {
            meta charset="utf-8";
            link rel="stylesheet" href="https://stackpath.bootstrapcdn.com/bootstrap/4.1.3/css/bootstrap.min.css";
            title {
                @if let Some(page_title) = page_title {
                    (page_title) " - "
                }
                "maud"
            }
        }
    }
}

pub(in crate::frontend) fn footer() -> Markup {
    html! {
        footer class="p-3 bg-dark text-white" {
            div.container {
                span { "Contact: dev@phansch.net" }
            }
        }
    }
}

pub(in crate::frontend) fn page(title: &Option<&str>, body: Markup, current_user: Option<User>) -> Markup {
    html! {
        (header(title))
        body {
            div.navbar.navbar-expand-lg.navbar-light.bg-light {
                div.container {
                    a.navbar-brand href="#" { "mend-rs" }
                    button.navbar-toggler type="button" data-toggle="collapse" data-target="#navbar-content" aria-controls="navbar-content" {
                        span.navbar-toggler-icon {}
                    }
                    div.collapse.navbar-collapse#navbar-content {
                        ul.navbar-nav.mr-auto {
                            @if current_user.is_some() {
                                li.nav-item {
                                    a.nav-link href="/" { "Dashboard" }
                                }
                                li.nav-item {
                                    a.nav-link href="/logout" { "Logout" }
                                }
                            } @else {
                                li.nav-item {
                                    a.nav-link href=(signup_link()) { "Login" }
                                }
                            }
                        }
                    }
                }
            }
            div.container {
                div.row {
                    (body)
                }
            }
        }
        (footer())
    }
}
