#[cfg(feature = "OcLgTriangleLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgTriangleLeft")]
/// *This icon requires the feature* `OcLgTriangleLeft` *to be enabled*.
#[component]
pub fn TriangleLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="m8.854 11.646 5.792-5.792a.5.5 0 0 1 .854.353v11.586a.5.5 0 0 1-.854.353l-5.792-5.792a.5.5 0 0 1 0-.708Z" /></svg>
   }
}