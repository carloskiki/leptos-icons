#[cfg(feature = "BiSolidArrowToBottom")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidArrowToBottom")]
/// *This icon requires the feature* `BiSolidArrowToBottom` *to be enabled*.
#[component]
pub fn ArrowToBottom(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 18h12v2H6zm5-14v6H6l6 6 6-6h-5V4z" /></svg>
   }
}