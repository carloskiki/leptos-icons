#[cfg(feature = "ImMobile")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "ImMobile")]
/// *This icon requires the feature* `ImMobile` *to be enabled*.
#[component]
pub fn Mobile(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" version="1.1" width="16" height="16" viewBox="0 0 16 16"><path fill="#000000" d="M11.5 0h-7c-0.825 0-1.5 0.675-1.5 1.5v13c0 0.825 0.675 1.5 1.5 1.5h7c0.825 0 1.5-0.675 1.5-1.5v-13c0-0.825-0.675-1.5-1.5-1.5zM6 0.75h4v0.5h-4v-0.5zM8 15c-0.552 0-1-0.448-1-1s0.448-1 1-1 1 0.448 1 1-0.448 1-1 1zM12 12h-8v-10h8v10z" /></svg>
   }
}