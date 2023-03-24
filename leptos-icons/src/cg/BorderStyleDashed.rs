#[cfg(feature = "CgBorderStyleDashed")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgBorderStyleDashed")]
/// *This icon requires the feature* `CgBorderStyleDashed` *to be enabled*.
#[component]
pub fn BorderStyleDashed(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M4 11H8V13H4V11Z" fill="currentColor" /><path d="M10 11H14V13H10V11Z" fill="currentColor" /><path d="M20 11H16V13H20V11Z" fill="currentColor" /></svg>
   }
}