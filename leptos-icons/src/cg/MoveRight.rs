#[cfg(feature = "CgMoveRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgMoveRight")]
/// *This icon requires the feature* `CgMoveRight` *to be enabled*.
#[component]
pub fn MoveRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M5 17V15H13V17H5Z" fill="currentColor" /><path d="M5 9V7H13V9H5Z" fill="currentColor" /><path d="M5 12.9999V10.9999H14.9999V7.96454L19.071 11.9644L14.9999 15.9644V12.9999H5Z" fill="currentColor" /></svg>
   }
}