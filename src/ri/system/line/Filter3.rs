#[cfg(feature = "RiSystemLineFilter3")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemLineFilter3")]
/// *This icon requires the feature* `RiSystemLineFilter3` *to be enabled*.
#[component]
pub fn Filter3(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M10 18h4v-2h-4v2zM3 6v2h18V6H3zm3 7h12v-2H6v2z" /></g></svg>
   }
}