#[cfg(feature = "VsMenu")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsMenu")]
/// *This icon requires the feature* `VsMenu` *to be enabled*.
#[component]
pub fn Menu(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M16 5H0V4h16v1zm0 8H0v-1h16v1zm0-4.008H0V8h16v.992z" /></svg>
   }
}