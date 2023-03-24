#[cfg(feature = "IoBarbellSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBarbellSharp")]
/// *This icon requires the feature* `IoBarbellSharp` *to be enabled*.
#[component]
pub fn BarbellSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="496 176 438 176 438 112 362 112 362 234 150 234 150 112 74 112 74 176 16 176 16 336 74 336 74 400 150 400 150 278 362 278 362 400 438 400 438 336 496 336 496 176" /></svg>
   }
}