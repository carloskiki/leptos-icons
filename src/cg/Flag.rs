#[cfg(feature = "CgFlag")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgFlag")]
/// *This icon requires the feature* `CgFlag` *to be enabled*.
#[component]
pub fn Flag(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M4 21H6V11H12V13H20V5H13V3H4V21ZM12 5H6V9H13V11H18V7H12V5Z" fill="currentColor" /></svg>
   }
}