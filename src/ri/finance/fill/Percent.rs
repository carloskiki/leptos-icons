#[cfg(feature = "RiFinanceFillPercent")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiFinanceFillPercent")]
/// *This icon requires the feature* `RiFinanceFillPercent` *to be enabled*.
#[component]
pub fn Percent(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M17.5 21a3.5 3.5 0 1 1 0-7 3.5 3.5 0 0 1 0 7zm-11-11a3.5 3.5 0 1 1 0-7 3.5 3.5 0 0 1 0 7zm12.571-6.485l1.414 1.414L4.93 20.485l-1.414-1.414L19.07 3.515z" /></g></svg>
   }
}