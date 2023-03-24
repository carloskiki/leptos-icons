#[cfg(feature = "VsGripper")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsGripper")]
/// *This icon requires the feature* `VsGripper` *to be enabled*.
#[component]
pub fn Gripper(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M5 3h2v2H5zm0 4h2v2H5zm0 4h2v2H5zm4-8h2v2H9zm0 4h2v2H9zm0 4h2v2H9z" /></svg>
   }
}