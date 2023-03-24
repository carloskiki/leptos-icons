#[cfg(feature = "IoTabletLandscapeSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoTabletLandscapeSharp")]
/// *This icon requires the feature* `IoTabletLandscapeSharp` *to be enabled*.
#[component]
pub fn TabletLandscapeSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M0,82V430a18,18,0,0,0,18,18H494a18,18,0,0,0,18-18V82a18,18,0,0,0-18-18H18A18,18,0,0,0,0,82ZM448,412H64V100H448Z" /></svg>
   }
}