#[cfg(feature = "CgArrowBottomRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgArrowBottomRight")]
/// *This icon requires the feature* `CgArrowBottomRight` *to be enabled*.
#[component]
pub fn ArrowBottomRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M5.75739 7.17154L7.1716 5.75732L16.2426 14.8283L16.2426 10.2427H18.2426L18.2426 18.2427H10.2426V16.2427L14.8285 16.2427L5.75739 7.17154Z" fill="currentColor" /></svg>
   }
}