#[cfg(feature = "VsAdd")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsAdd")]
/// *This icon requires the feature* `VsAdd` *to be enabled*.
#[component]
pub fn Add(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M14 7v1H8v6H7V8H1V7h6V1h1v6h6z" /></svg>
   }
}