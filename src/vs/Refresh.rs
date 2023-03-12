#[cfg(feature = "VsRefresh")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsRefresh")]
/// *This icon requires the feature* `VsRefresh` *to be enabled*.
#[component]
pub fn Refresh(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M4.681 3H2V2h3.5l.5.5V6H5V4a5 5 0 1 0 4.53-.761l.302-.954A6 6 0 1 1 4.681 3z" /></svg>
   }
}