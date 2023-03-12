#[cfg(feature = "CgAlignMiddle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgAlignMiddle")]
/// *This icon requires the feature* `CgAlignMiddle` *to be enabled*.
#[component]
pub fn AlignMiddle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M13 9H17V15H13V9Z" fill="currentColor" fill-opacity="0.5" /><path d="M7 6H11V18H7V6Z" fill="currentColor" /></svg>
   }
}