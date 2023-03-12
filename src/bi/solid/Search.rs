#[cfg(feature = "BiSolidSearch")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidSearch")]
/// *This icon requires the feature* `BiSolidSearch` *to be enabled*.
#[component]
pub fn Search(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M10 2c-4.411 0-8 3.589-8 8s3.589 8 8 8a7.952 7.952 0 0 0 4.897-1.688l4.396 4.396 1.414-1.414-4.396-4.396A7.952 7.952 0 0 0 18 10c0-4.411-3.589-8-8-8z" /></svg>
   }
}