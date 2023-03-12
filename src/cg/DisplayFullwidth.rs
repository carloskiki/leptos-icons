#[cfg(feature = "CgDisplayFullwidth")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgDisplayFullwidth")]
/// *This icon requires the feature* `CgDisplayFullwidth` *to be enabled*.
#[component]
pub fn DisplayFullwidth(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M2 5H22V3H2V5Z" fill="currentColor" /><path d="M2 21H22V19H2V21Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M2 7V17H22V7H2ZM4 9H20V15H4V9Z" fill="currentColor" /></svg>
   }
}