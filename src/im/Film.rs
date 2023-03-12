#[cfg(feature = "ImFilm")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImFilm")]
/// *This icon requires the feature* `ImFilm` *to be enabled*.
#[component]
pub fn Film(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 2v12h16v-12h-16zM3 13h-2v-2h2v2zM3 9h-2v-2h2v2zM3 5h-2v-2h2v2zM12 13h-8v-10h8v10zM15 13h-2v-2h2v2zM15 9h-2v-2h2v2zM15 5h-2v-2h2v2zM6 5v6l4-3z" /></svg>
   }
}