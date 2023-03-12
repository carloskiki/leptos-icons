#[cfg(feature = "BiSolidCircleHalf")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCircleHalf")]
/// *This icon requires the feature* `BiSolidCircleHalf` *to be enabled*.
#[component]
pub fn CircleHalf(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 2h-1v20h1a10 10 0 0 0 0-20z" /></svg>
   }
}