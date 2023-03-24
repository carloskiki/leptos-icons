#[cfg(feature = "ImLaptop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImLaptop")]
/// *This icon requires the feature* `ImLaptop` *to be enabled*.
#[component]
pub fn Laptop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M14 11v-8c0-0.55-0.45-1-1-1h-10c-0.55 0-1 0.45-1 1v8h-2v3h16v-3h-2zM10 13h-4v-1h4v1zM13 11h-10v-7.998c0.001-0.001 0.001-0.001 0.002-0.002h9.996c0.001 0.001 0.001 0.001 0.002 0.002v7.998z" /></svg>
   }
}