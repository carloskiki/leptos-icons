#[cfg(feature = "IoCaretUpOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCaretUpOutline")]
/// *This icon requires the feature* `IoCaretUpOutline` *to be enabled*.
#[component]
pub fn CaretUpOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M414,321.94,274.22,158.82a24,24,0,0,0-36.44,0L98,321.94c-13.34,15.57-2.28,39.62,18.22,39.62H395.82C416.32,361.56,427.38,337.51,414,321.94Z" /></svg>
   }
}