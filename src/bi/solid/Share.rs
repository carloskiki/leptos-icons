#[cfg(feature = "BiSolidShare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidShare")]
/// *This icon requires the feature* `BiSolidShare` *to be enabled*.
#[component]
pub fn Share(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11 6.914V2.586L6.293 7.293l-3.774 3.774 3.841 3.201L11 18.135V13.9c8.146-.614 11 4.1 11 4.1 0-2.937-.242-5.985-2.551-8.293C16.765 7.022 12.878 6.832 11 6.914z" /></svg>
   }
}