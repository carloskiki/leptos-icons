#[cfg(feature = "BiSolidArrowFromTop")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidArrowFromTop")]
/// *This icon requires the feature* `BiSolidArrowFromTop` *to be enabled*.
#[component]
pub fn ArrowFromTop(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6 4h12v2H6zm5 4v6H6l6 6 6-6h-5V8z" /></svg>
   }
}