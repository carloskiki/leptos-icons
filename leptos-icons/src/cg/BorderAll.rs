#[cfg(feature = "CgBorderAll")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgBorderAll")]
/// *This icon requires the feature* `CgBorderAll` *to be enabled*.
#[component]
pub fn BorderAll(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M6.5 6.5H17.5V17.5H6.5V6.5Z" stroke="currentColor" stroke-width="3" /></svg>
   }
}