#[cfg(feature = "CgUnfold")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgUnfold")]
/// *This icon requires the feature* `CgUnfold` *to be enabled*.
#[component]
pub fn Unfold(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M2 12C2 17.5228 6.47715 22 12 22C17.5228 22 22 17.5228 22 12H2Z" fill="currentColor" /></svg>
   }
}