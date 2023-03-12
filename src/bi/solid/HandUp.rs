#[cfg(feature = "BiSolidHandUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidHandUp")]
/// *This icon requires the feature* `BiSolidHandUp` *to be enabled*.
#[component]
pub fn HandUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19.221 10.803 12 10V4a2 2 0 0 0-4 0v12l-3.031-1.212a2 2 0 0 0-2.64 1.225l-.113.34a.998.998 0 0 0 .309 1.084l5.197 4.332c.179.149.406.231.64.231H19a2 2 0 0 0 2-2v-7.21a2 2 0 0 0-1.779-1.987z" /></svg>
   }
}