#[cfg(feature = "VsBlank")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsBlank")]
/// *This icon requires the feature* `VsBlank` *to be enabled*.
#[component]
pub fn Blank(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor" />
   }
}