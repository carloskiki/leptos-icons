#[cfg(feature = "ImAidKit")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImAidKit")]
/// *This icon requires the feature* `ImAidKit` *to be enabled*.
#[component]
pub fn AidKit(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M14 4h-3v-2c0-0.55-0.45-1-1-1h-4c-0.55 0-1 0.45-1 1v2h-3c-1.1 0-2 0.9-2 2v8c0 1.1 0.9 2 2 2h12c1.1 0 2-0.9 2-2v-8c0-1.1-0.9-2-2-2zM6 2h4v2h-4v-2zM12 11h-3v3h-2v-3h-3v-2h3v-3h2v3h3v2z" /></svg>
   }
}