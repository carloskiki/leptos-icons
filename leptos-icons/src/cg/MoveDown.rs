#[cfg(feature = "CgMoveDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgMoveDown")]
/// *This icon requires the feature* `CgMoveDown` *to be enabled*.
#[component]
pub fn MoveDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M7 5H9V13H7V5Z" fill="currentColor" /><path d="M15 5H17V13H15V5Z" fill="currentColor" /><path d="M11.0001 5H13.0001V14.9999H16.0355L12.0356 19.071L8.03564 14.9999H11.0001V5Z" fill="currentColor" /></svg>
   }
}