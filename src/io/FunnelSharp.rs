#[cfg(feature = "IoFunnelSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoFunnelSharp")]
/// *This icon requires the feature* `IoFunnelSharp` *to be enabled*.
#[component]
pub fn FunnelSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><polygon points="0 48 192 288 192 416 320 464 320 288 512 48 0 48" /></svg>
   }
}