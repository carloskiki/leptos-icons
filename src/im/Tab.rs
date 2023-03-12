#[cfg(feature = "ImTab")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImTab")]
/// *This icon requires the feature* `ImTab` *to be enabled*.
#[component]
pub fn Tab(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M15 0h1v8h-1v-8z" /><path fill="#000000" d="M0 8h1v8h-1v-8z" /><path fill="#000000" d="M5 11h11v2h-11v2.5l-3.5-3.5 3.5-3.5v2.5z" /><path fill="#000000" d="M11 5h-11v-2h11v-2.5l3.5 3.5-3.5 3.5z" /></svg>
   }
}