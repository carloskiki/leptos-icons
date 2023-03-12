#[cfg(feature = "FiCornerDownLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiCornerDownLeft")]
/// *This icon requires the feature* `FiCornerDownLeft` *to be enabled*.
#[component]
pub fn CornerDownLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="9 10 4 15 9 20" /><path d="M20 4v7a4 4 0 0 1-4 4H4" /></svg>
   }
}