#[cfg(feature = "IoManSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoManSharp")]
/// *This icon requires the feature* `IoManSharp` *to be enabled*.
#[component]
pub fn ManSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><circle cx="256" cy="56" r="56" /><path d="M336,128H176a32,32,0,0,0-32,32V320h48V192h8V512h52V328h8V512h52V192h8V320h48V160A32,32,0,0,0,336,128Z" /></svg>
   }
}