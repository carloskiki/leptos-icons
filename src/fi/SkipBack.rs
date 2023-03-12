#[cfg(feature = "FiSkipBack")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiSkipBack")]
/// *This icon requires the feature* `FiSkipBack` *to be enabled*.
#[component]
pub fn SkipBack(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="19 20 9 12 19 4 19 20" /><line x1="5" y1="19" x2="5" y2="5" /></svg>
   }
}