#[cfg(feature = "BsHCircleFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsHCircleFill")]
/// *This icon requires the feature* `BsHCircleFill` *to be enabled*.
#[component]
pub fn HCircleFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-h-circle-fill" viewBox="0 0 16 16"><path d="M16 8A8 8 0 1 1 0 8a8 8 0 0 1 16 0Zm-5-3.998H9.67v3.322H6.33V4.002H5V12h1.33V8.455h3.34V12H11V4.002Z" /></svg>
   }
}