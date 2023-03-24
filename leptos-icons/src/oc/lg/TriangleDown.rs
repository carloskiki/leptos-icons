#[cfg(feature = "OcLgTriangleDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "OcLgTriangleDown")]
/// *This icon requires the feature* `OcLgTriangleDown` *to be enabled*.
#[component]
pub fn TriangleDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M11.646 15.146 5.854 9.354a.5.5 0 0 1 .353-.854h11.586a.5.5 0 0 1 .353.854l-5.793 5.792a.5.5 0 0 1-.707 0Z" /></svg>
   }
}