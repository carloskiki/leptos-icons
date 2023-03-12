#[cfg(feature = "BiSolidToTop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidToTop")]
/// *This icon requires the feature* `BiSolidToTop` *to be enabled*.
#[component]
pub fn ToTop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 15h4v6h6v-6h4l-7-8zM4 3h16v2H4z" /></svg>
   }
}