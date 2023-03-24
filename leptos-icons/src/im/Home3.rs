#[cfg(feature = "ImHome3")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImHome3")]
/// *This icon requires the feature* `ImHome3` *to be enabled*.
#[component]
pub fn Home3(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M16 9.5l-3-3v-4.5h-2v2.5l-3-3-8 8v0.5h2v5h5v-3h2v3h5v-5h2z" /></svg>
   }
}