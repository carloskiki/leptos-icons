#[cfg(feature = "RiSystemFillFilter2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillFilter2")]
/// *This icon requires the feature* `RiSystemFillFilter2` *to be enabled*.
#[component]
pub fn Filter2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M10 14L4 5V3h16v2l-6 9v6l-4 2z" /></g></svg>
   }
}