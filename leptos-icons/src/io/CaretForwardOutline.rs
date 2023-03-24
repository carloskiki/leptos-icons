#[cfg(feature = "IoCaretForwardOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCaretForwardOutline")]
/// *This icon requires the feature* `IoCaretForwardOutline` *to be enabled*.
#[component]
pub fn CaretForwardOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M190.06,414,353.18,274.22a24,24,0,0,0,0-36.44L190.06,98c-15.57-13.34-39.62-2.28-39.62,18.22V395.82C150.44,416.32,174.49,427.38,190.06,414Z" /></svg>
   }
}