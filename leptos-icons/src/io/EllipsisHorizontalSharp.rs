#[cfg(feature = "IoEllipsisHorizontalSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoEllipsisHorizontalSharp")]
/// *This icon requires the feature* `IoEllipsisHorizontalSharp` *to be enabled*.
#[component]
pub fn EllipsisHorizontalSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="256" cy="256" r="48" /><circle cx="416" cy="256" r="48" /><circle cx="96" cy="256" r="48" /></svg>
   }
}