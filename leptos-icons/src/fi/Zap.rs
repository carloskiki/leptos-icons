#[cfg(feature = "FiZap")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiZap")]
/// *This icon requires the feature* `FiZap` *to be enabled*.
#[component]
pub fn Zap(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" /></svg>
   }
}