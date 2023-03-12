#[cfg(feature = "CgSpaceBetweenV")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgSpaceBetweenV")]
/// *This icon requires the feature* `CgSpaceBetweenV` *to be enabled*.
#[component]
pub fn SpaceBetweenV(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M5 5V9H19V5H17V7H7V5H5Z" fill="currentColor" /><path d="M5 19V15H19V19H17V17H7V19H5Z" fill="currentColor" /><path d="M7 11H17V13H7V11Z" fill="currentColor" /></svg>
   }
}