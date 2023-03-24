#[cfg(feature = "OcLgDash")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgDash")]
/// *This icon requires the feature* `OcLgDash` *to be enabled*.
#[component]
pub fn Dash(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M4.5 12.75a.75.75 0 0 1 .75-.75h13.5a.75.75 0 0 1 0 1.5H5.25a.75.75 0 0 1-.75-.75Z" /></svg>
   }
}