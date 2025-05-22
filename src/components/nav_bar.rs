use leptos::prelude::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
      <nav class="hstack">
        <h1>"IT Ticket Tracking System"</h1>
        <ul>
          <li><a href="/">Home</a></li>
          <li><a href="/login">Log out</a></li>
        </ul>
      </nav>
    }
}
