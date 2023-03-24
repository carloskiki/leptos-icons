#[cfg(feature = "CgShield")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgShield")]
/// *This icon requires the feature* `CgShield` *to be enabled*.
#[component]
pub fn Shield(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path fill-rule="evenodd" clip-rule="evenodd" d="M7 8V13C7 15.7614 9.23858 18 12 18C14.7614 18 17 15.7614 17 13V8H7ZM5 4H19V13C19 16.866 15.866 20 12 20C8.13401 20 5 16.866 5 13V4Z" fill="currentColor" /></svg>
   }
}