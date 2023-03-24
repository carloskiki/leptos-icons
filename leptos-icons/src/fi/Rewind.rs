#[cfg(feature = "FiRewind")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiRewind")]
/// *This icon requires the feature* `FiRewind` *to be enabled*.
#[component]
pub fn Rewind(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="11 19 2 12 11 5 11 19" /><polygon points="22 19 13 12 22 5 22 19" /></svg>
   }
}