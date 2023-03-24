#[cfg(feature = "VsThreeBars")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsThreeBars")]
/// *This icon requires the feature* `VsThreeBars` *to be enabled*.
#[component]
pub fn ThreeBars(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" clip-rule="evenodd" d="M14 5H2V3h12v2zm0 4H2V7h12v2zM2 13h12v-2H2v2z" /></svg>
   }
}