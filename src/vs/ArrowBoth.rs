#[cfg(feature = "VsArrowBoth")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsArrowBoth")]
/// *This icon requires the feature* `VsArrowBoth` *to be enabled*.
#[component]
pub fn ArrowBoth(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M3 9l2.146 2.146-.707.708-3-3v-.708l3-3 .707.708L3 8h10l-2.146-2.146.707-.708 3 3v.708l-3 3-.707-.707L13 9H3z" /></svg>
   }
}