#[cfg(feature = "BiRegularItalic")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularItalic")]
/// *This icon requires the feature* `BiRegularItalic` *to be enabled*.
#[component]
pub fn Italic(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M19 7V4H9v3h2.868L9.012 17H5v3h10v-3h-2.868l2.856-10z" /></svg>
   }
}