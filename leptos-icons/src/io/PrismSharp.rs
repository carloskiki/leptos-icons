#[cfg(feature = "IoPrismSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoPrismSharp")]
/// *This icon requires the feature* `IoPrismSharp` *to be enabled*.
#[component]
pub fn PrismSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" id="icons" viewBox="0 0 512 512"><path d="M256,16,16,352,256,496,496,352Zm-20,96.82V437.35L73.73,340Z" /></svg>
   }
}