#[cfg(feature = "BsThreeDotsVertical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsThreeDotsVertical")]
/// *This icon requires the feature* `BsThreeDotsVertical` *to be enabled*.
#[component]
pub fn ThreeDotsVertical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-three-dots-vertical" viewBox="0 0 16 16"><path d="M9.5 13a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0zm0-5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0zm0-5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0z" /></svg>
   }
}