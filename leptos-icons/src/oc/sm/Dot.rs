#[cfg(feature = "OcSmDot")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcSmDot")]
/// *This icon requires the feature* `OcSmDot` *to be enabled*.
#[component]
pub fn Dot(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path d="M4 8a4 4 0 1 1 8 0 4 4 0 0 1-8 0Zm4-2.5a2.5 2.5 0 1 0 0 5 2.5 2.5 0 0 0 0-5Z" /></svg>
   }
}