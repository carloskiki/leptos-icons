#[cfg(feature = "FiSkipForward")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiSkipForward")]
/// *This icon requires the feature* `FiSkipForward` *to be enabled*.
#[component]
pub fn SkipForward(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="5 4 15 12 5 20 5 4" /><line x1="19" y1="5" x2="19" y2="19" /></svg>
   }
}