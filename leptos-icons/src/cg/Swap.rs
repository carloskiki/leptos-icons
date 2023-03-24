#[cfg(feature = "CgSwap")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgSwap")]
/// *This icon requires the feature* `CgSwap` *to be enabled*.
#[component]
pub fn Swap(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M16 13V11.5H10V9.5H16V8L19 10.5L16 13Z" fill="currentColor" /><path d="M8 17V15.5H14V13.5H8V12L5 14.5L8 17Z" fill="currentColor" /></svg>
   }
}