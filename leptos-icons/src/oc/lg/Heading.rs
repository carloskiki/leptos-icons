#[cfg(feature = "OcLgHeading")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgHeading")]
/// *This icon requires the feature* `OcLgHeading` *to be enabled*.
#[component]
pub fn Heading(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M6.25 4a.75.75 0 0 1 .75.75V11h10V4.75a.75.75 0 0 1 1.5 0v14.5a.75.75 0 0 1-1.5 0V12.5H7v6.75a.75.75 0 0 1-1.5 0V4.75A.75.75 0 0 1 6.25 4Z" /></svg>
   }
}