#[cfg(feature = "BsDot")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsDot")]
/// *This icon requires the feature* `BsDot` *to be enabled*.
#[component]
pub fn Dot(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-dot" viewBox="0 0 16 16"><path d="M8 9.5a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3z" /></svg>
   }
}