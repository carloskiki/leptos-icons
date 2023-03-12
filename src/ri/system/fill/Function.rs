#[cfg(feature = "RiSystemFillFunction")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiSystemFillFunction")]
/// *This icon requires the feature* `RiSystemFillFunction` *to be enabled*.
#[component]
pub fn Function(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M3 3h8v8H3V3zm0 10h8v8H3v-8zM13 3h8v8h-8V3zm0 10h8v8h-8v-8z" /></g></svg>
   }
}