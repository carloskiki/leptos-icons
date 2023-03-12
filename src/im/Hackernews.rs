#[cfg(feature = "ImHackernews")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImHackernews")]
/// *This icon requires the feature* `ImHackernews` *to be enabled*.
#[component]
pub fn Hackernews(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M0 0v16h16v-16h-16zM8.5 9.125v3.375h-1v-3.375l-2.734-5.125h1.134l2.1 3.938 2.1-3.938h1.134l-2.734 5.125z" /></svg>
   }
}