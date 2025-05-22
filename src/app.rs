use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Redirect, Route, Router, Routes},
    path,
};

use crate::components::{ticket::TicketsPage, NavBar, NotFound};

#[allow(non_snake_case)]
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
      // injects a stylesheet into the document <head>
      // id=leptos means cargo-leptos will hot-reload this stylesheet
      <Stylesheet id="leptos" href="/pkg/ittts.css" />
      <Title text="ITTTS" />
      <header> <NavBar /> </header>
      <Router>
        <main>
          <Routes fallback=move || "Not found.">
            <Route path=path!("/") view=|| view! { <Redirect path="/tickets" /> } />
            <Route path=path!("/tickets") view=TicketsPage />
            <Route path=path!("/*any") view=NotFound />
          </Routes>
        </main>
      </Router>
    }
}
