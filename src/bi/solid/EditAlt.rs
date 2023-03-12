#[cfg(feature = "BiSolidEditAlt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidEditAlt")]
/// *This icon requires the feature* `BiSolidEditAlt` *to be enabled*.
#[component]
pub fn EditAlt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m16 2.012 3 3L16.713 7.3l-3-3zM4 14v3h3l8.299-8.287-3-3zm0 6h16v2H4z" /></svg>
   }
}