#[cfg(feature = "RiLogosLineMicrosoft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiLogosLineMicrosoft")]
/// *This icon requires the feature* `RiLogosLineMicrosoft` *to be enabled*.
#[component]
pub fn Microsoft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M11 5H5v6h6V5zm2 0v6h6V5h-6zm6 8h-6v6h6v-6zm-8 6v-6H5v6h6zM3 3h18v18H3V3z" /></g></svg>
   }
}