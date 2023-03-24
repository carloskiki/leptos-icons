#[cfg(feature = "CgSpinnerAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgSpinnerAlt")]
/// *This icon requires the feature* `CgSpinnerAlt` *to be enabled*.
#[component]
pub fn SpinnerAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M2 12C2 6.47715 6.47715 2 12 2V5C8.13401 5 5 8.13401 5 12H2Z" fill="currentColor" /></svg>
   }
}