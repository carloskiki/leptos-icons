#[cfg(feature = "VsListFlat")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsListFlat")]
/// *This icon requires the feature* `VsListFlat` *to be enabled*.
#[component]
pub fn ListFlat(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M2 10V9h12v1H2zm0-4h12v1H2V6zm12-3v1H2V3h12zM2 12v1h12v-1H2z" /></svg>
   }
}