#[cfg(feature = "RiSystemLineMenu4")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemLineMenu4")]
/// *This icon requires the feature* `RiSystemLineMenu4` *to be enabled*.
#[component]
pub fn Menu4(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M16 18v2H5v-2h11zm5-7v2H3v-2h18zm-2-7v2H8V4h11z" /></g></svg>
   }
}