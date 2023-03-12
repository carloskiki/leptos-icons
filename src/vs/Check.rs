#[cfg(feature = "VsCheck")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsCheck")]
/// *This icon requires the feature* `VsCheck` *to be enabled*.
#[component]
pub fn Check(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M14.431 3.323l-8.47 10-.79-.036-3.35-4.77.818-.574 2.978 4.24 8.051-9.506.764.646z" /></svg>
   }
}