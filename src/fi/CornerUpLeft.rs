#[cfg(feature = "FiCornerUpLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiCornerUpLeft")]
/// *This icon requires the feature* `FiCornerUpLeft` *to be enabled*.
#[component]
pub fn CornerUpLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 14 4 9 9 4" /><path d="M20 20v-7a4 4 0 0 0-4-4H4" /></svg>
   }
}