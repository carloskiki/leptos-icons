#[cfg(feature = "RiSystemLineSubtract")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemLineSubtract")]
/// *This icon requires the feature* `RiSystemLineSubtract` *to be enabled*.
#[component]
pub fn Subtract(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M5 11h14v2H5z" /></g></svg>
   }
}