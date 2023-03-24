#[cfg(feature = "BsFastForwardCircleFill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsFastForwardCircleFill")]
/// *This icon requires the feature* `BsFastForwardCircleFill` *to be enabled*.
#[component]
pub fn FastForwardCircleFill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-fast-forward-circle-fill" viewBox="0 0 16 16"><path d="M8 16A8 8 0 1 0 8 0a8 8 0 0 0 0 16ZM4.79 5.093 8 7.386V5.5a.5.5 0 0 1 .79-.407l3.5 2.5a.5.5 0 0 1 0 .814l-3.5 2.5A.5.5 0 0 1 8 10.5V8.614l-3.21 2.293A.5.5 0 0 1 4 10.5v-5a.5.5 0 0 1 .79-.407Z" /></svg>
   }
}