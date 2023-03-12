#[cfg(feature = "BiRegularPause")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularPause")]
/// *This icon requires the feature* `BiRegularPause` *to be enabled*.
#[component]
pub fn Pause(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M8 7h3v10H8zm5 0h3v10h-3z" /></svg>
   }
}