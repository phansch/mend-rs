extern crate maud;
use self::maud::{html, Markup};
use crate::frontend::partials::page;
use crate::models::User;

pub fn render(current_user: Option<User>) -> Markup {
    page(
        &Some("Hello!"),
        html! {
            @if let Some(user) = &current_user {
                h1 { "repo list coming soon, " (user.username) }
                a.btn.btn-primary href="https://github.com/apps/mend-rs/installations/new" { "get started" }
            }
        },
        current_user
    )
}
