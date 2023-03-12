#[cfg(feature = "RiDesignFillClockwise")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDesignFillClockwise")]
/// *This icon requires the feature* `RiDesignFillClockwise` *to be enabled*.
#[component]
pub fn Clockwise(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M20 10h3l-4 5-4-5h3V8a3 3 0 0 0-3-3h-4V3h4a5 5 0 0 1 5 5v2zm-7-1a1 1 0 0 1 1 1v10a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V10a1 1 0 0 1 1-1h10z" /></g></svg>
   }
}