#[cfg(feature = "IoBackspaceSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoBackspaceSharp")]
/// *This icon requires the feature* `IoBackspaceSharp` *to be enabled*.
#[component]
pub fn BackspaceSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M144,96,32,256,144,416H448V96ZM359.3,322.34,336.67,345l-65-65-65,65L184,322.34l65-65-65-65,22.63-22.63,65,65,65-65,22.63,22.63-65,65Z" /></svg>
   }
}