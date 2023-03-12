#[cfg(feature = "FiPlay")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiPlay")]
/// *This icon requires the feature* `FiPlay` *to be enabled*.
#[component]
pub fn Play(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="5 3 19 12 5 21 5 3" /></svg>
   }
}