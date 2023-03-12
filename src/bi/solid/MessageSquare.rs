#[cfg(feature = "BiSolidMessageSquare")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMessageSquare")]
/// *This icon requires the feature* `BiSolidMessageSquare` *to be enabled*.
#[component]
pub fn MessageSquare(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M16 2H8C4.691 2 2 4.691 2 8v13a1 1 0 0 0 1 1h13c3.309 0 6-2.691 6-6V8c0-3.309-2.691-6-6-6z" /></svg>
   }
}