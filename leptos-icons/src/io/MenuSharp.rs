#[cfg(feature = "IoMenuSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoMenuSharp")]
/// *This icon requires the feature* `IoMenuSharp` *to be enabled*.
#[component]
pub fn MenuSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M64,384H448V341.33H64Zm0-106.67H448V234.67H64ZM64,128v42.67H448V128Z" /></svg>
   }
}