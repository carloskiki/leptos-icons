#[cfg(feature = "RiSystemFillMenu2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillMenu2")]
/// *This icon requires the feature* `RiSystemFillMenu2` *to be enabled*.
#[component]
pub fn Menu2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M3 4h18v2H3V4zm0 7h12v2H3v-2zm0 7h18v2H3v-2z" /></g></svg>
   }
}