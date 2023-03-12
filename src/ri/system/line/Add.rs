#[cfg(feature = "RiSystemLineAdd")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemLineAdd")]
/// *This icon requires the feature* `RiSystemLineAdd` *to be enabled*.
#[component]
pub fn Add(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M11 11V5h2v6h6v2h-6v6h-2v-6H5v-2z" /></g></svg>
   }
}