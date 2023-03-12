#[cfg(feature = "FaSolidEllipsisVertical")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidEllipsisVertical")]
/// *This icon requires the feature* `FaSolidEllipsisVertical` *to be enabled*.
#[component]
pub fn EllipsisVertical(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 128 512"><path d="M56 472a56 56 0 1 1 0-112 56 56 0 1 1 0 112zm0-160a56 56 0 1 1 0-112 56 56 0 1 1 0 112zM0 96a56 56 0 1 1 112 0A56 56 0 1 1 0 96z" /></svg>
   }
}