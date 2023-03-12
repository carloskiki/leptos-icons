#[cfg(feature = "CgPushChevronLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPushChevronLeft")]
/// *This icon requires the feature* `CgPushChevronLeft` *to be enabled*.
#[component]
pub fn PushChevronLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M16.929 5L18.3432 6.41421L12.6863 12.0711L18.3432 17.7279L16.929 19.1421L9.85789 12.0711L16.929 5Z" fill="currentColor" /><path d="M8 19V5H6V19H8Z" fill="currentColor" /></svg>
   }
}