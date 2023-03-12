#[cfg(feature = "VsListFlat")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsListFlat")]
/// *This icon requires the feature* `VsListFlat` *to be enabled*.
#[component]
pub fn ListFlat(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><rect x="2" y="9" width="9" height="1" /><rect x="2" y="12" width="8" height="1" /><rect x="2" y="6" width="12" height="1" /><rect x="2" y="3" width="11" height="1" /></svg>
   }
}