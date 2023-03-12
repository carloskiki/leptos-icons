#[cfg(feature = "CgPathBack")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPathBack")]
/// *This icon requires the feature* `CgPathBack` *to be enabled*.
#[component]
pub fn PathBack(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M9 14H4V4H14V9H19V19H9V14ZM6 6H12V12H6V6Z" fill="currentColor" /></svg>
   }
}