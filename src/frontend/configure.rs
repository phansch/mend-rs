extern crate maud;
use self::maud::{html, Markup};
use crate::frontend::partials::page;
use crate::models::{User, Repo};

pub fn render(current_user: Option<User>, repos: Vec<Repo>) -> Markup {
    page(
        &Some("Hello!"),
        html! {
            ul.list-group {
                @for repo in repos {
                    div {
                        span { (repo.name) }
                        span {
                            button { "Activate" }
                        }
                    }
                }
            }
        },
        current_user
    )
}
