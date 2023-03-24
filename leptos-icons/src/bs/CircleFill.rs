#[cfg(feature = "BsCircleFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsCircleFill")]
/// *This icon requires the feature* `BsCircleFill` *to be enabled*.
#[component]
pub fn CircleFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-circle-fill" viewBox="0 0 16 16"><circle cx="8" cy="8" r="8" /></svg>
   }
}