#[cfg(feature = "CgBmw")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgBmw")]
/// *This icon requires the feature* `CgBmw` *to be enabled*.
#[component]
pub fn Bmw(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12ZM5 12C5 15.866 8.13401 19 12 19V12H19C19 8.13401 15.866 5 12 5V12H5Z" fill="currentColor" /></svg>
   }
}