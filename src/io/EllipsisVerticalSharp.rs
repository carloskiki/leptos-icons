#[cfg(feature = "IoEllipsisVerticalSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoEllipsisVerticalSharp")]
/// *This icon requires the feature* `IoEllipsisVerticalSharp` *to be enabled*.
#[component]
pub fn EllipsisVerticalSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="256" cy="256" r="48" /><circle cx="256" cy="416" r="48" /><circle cx="256" cy="96" r="48" /></svg>
   }
}