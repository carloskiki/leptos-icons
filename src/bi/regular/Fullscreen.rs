#[cfg(feature = "BiRegularFullscreen")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularFullscreen")]
/// *This icon requires the feature* `BiRegularFullscreen` *to be enabled*.
#[component]
pub fn Fullscreen(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 5h5V3H3v7h2zm5 14H5v-5H3v7h7zm11-5h-2v5h-5v2h7zm-2-4h2V3h-7v2h5z" /></svg>
   }
}