#[cfg(feature = "FiCornerDownRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FiCornerDownRight")]
/// *This icon requires the feature* `FiCornerDownRight` *to be enabled*.
#[component]
pub fn CornerDownRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="15 10 20 15 15 20" /><path d="M4 4v7a4 4 0 0 0 4 4h12" /></svg>
   }
}