#[cfg(feature = "AiOutlinedEllipsis")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiOutlinedEllipsis")]
/// *This icon requires the feature* `AiOutlinedEllipsis` *to be enabled*.
#[component]
pub fn Ellipsis(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" viewBox="0 0 1024 1024"><path d="M176 511a56 56 0 1 0 112 0 56 56 0 1 0-112 0zm280 0a56 56 0 1 0 112 0 56 56 0 1 0-112 0zm280 0a56 56 0 1 0 112 0 56 56 0 1 0-112 0z" /></svg>
   }
}