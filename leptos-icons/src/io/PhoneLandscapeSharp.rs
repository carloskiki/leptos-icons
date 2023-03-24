#[cfg(feature = "IoPhoneLandscapeSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPhoneLandscapeSharp")]
/// *This icon requires the feature* `IoPhoneLandscapeSharp` *to be enabled*.
#[component]
pub fn PhoneLandscapeSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M0,130V382a18,18,0,0,0,18,18H494a18,18,0,0,0,18-18V130a18,18,0,0,0-18-18H18A18,18,0,0,0,0,130ZM448,364H64V148H448Z" /></svg>
   }
}