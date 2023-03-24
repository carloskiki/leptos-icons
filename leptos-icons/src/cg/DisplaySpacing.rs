#[cfg(feature = "CgDisplaySpacing")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgDisplaySpacing")]
/// *This icon requires the feature* `CgDisplaySpacing` *to be enabled*.
#[component]
pub fn DisplaySpacing(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M3 21V3H5V21H3Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M7 3H17V21H7V3ZM9 5V19H15V5H9Z" fill="currentColor" /><path d="M19 3V21H21V3H19Z" fill="currentColor" /></svg>
   }
}