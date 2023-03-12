#[cfg(feature = "AiOutlinedMore")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "AiOutlinedMore")]
/// *This icon requires the feature* `AiOutlinedMore` *to be enabled*.
#[component]
pub fn More(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon" width="200" height="200" viewBox="0 0 1024 1024"><path fill="#333" d="M456 231a56 56 0 1 0 112 0 56 56 0 1 0-112 0zm0 280a56 56 0 1 0 112 0 56 56 0 1 0-112 0zm0 280a56 56 0 1 0 112 0 56 56 0 1 0-112 0z" /></svg>
   }
}