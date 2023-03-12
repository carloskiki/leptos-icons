#[cfg(feature = "VsListTree")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsListTree")]
/// *This icon requires the feature* `VsListTree` *to be enabled*.
#[component]
pub fn ListTree(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><rect x="4" y="9" width="9" height="1" /><rect x="4" y="12" width="7" height="1" /><rect x="4" y="6" width="10" height="1" /><rect x="1" y="3" width="11" height="1" /><rect x="4" y="4" width="1" height="9" /></svg>
   }
}