#[cfg(feature = "BiRegularExpandAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularExpandAlt")]
/// *This icon requires the feature* `BiRegularExpandAlt` *to be enabled*.
#[component]
pub fn ExpandAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M5 12H3v9h9v-2H5zm7-7h7v7h2V3h-9z" /></svg>
   }
}