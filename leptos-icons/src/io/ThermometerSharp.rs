#[cfg(feature = "IoThermometerSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoThermometerSharp")]
/// *This icon requires the feature* `IoThermometerSharp` *to be enabled*.
#[component]
pub fn ThermometerSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M320,291.24V80a64,64,0,1,0-128,0V291.24A113.39,113.39,0,0,0,144,384a112,112,0,0,0,224,0A113.39,113.39,0,0,0,320,291.24ZM256,432a48,48,0,0,1-16-93.26V96h32V338.74A48,48,0,0,1,256,432Z" /></svg>
   }
}