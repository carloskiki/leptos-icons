#[cfg(feature = "IoBagHandleOutline")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBagHandleOutline")]
/// *This icon requires the feature* `IoBagHandleOutline` *to be enabled*.
#[component]
pub fn BagHandleOutline(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" id="icons" viewBox="0 0 512 512"><path d="M80,176a16,16,0,0,0-16,16V408c0,30.24,25.76,56,56,56H392c30.24,0,56-24.51,56-54.75V192a16,16,0,0,0-16-16Z" fill="none" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" /><path d="M160,176V144a96,96,0,0,1,96-96h0a96,96,0,0,1,96,96v32" fill="none" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" /><path d="M160,224v16a96,96,0,0,0,96,96h0a96,96,0,0,0,96-96V224" fill="none" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="32" /></svg>
   }
}