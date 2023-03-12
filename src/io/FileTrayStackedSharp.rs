#[cfg(feature = "IoFileTrayStackedSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoFileTrayStackedSharp")]
/// *This icon requires the feature* `IoFileTrayStackedSharp` *to be enabled*.
#[component]
pub fn FileTrayStackedSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M448,16H64L32,176V320H480V176ZM436,176H320a64,64,0,0,1-128,0H76L98,58H414Z" /><path d="M320,352a64,64,0,0,1-128,0H32V496H480V352Z" /></svg>
   }
}