#[cfg(feature = "TbX")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbX")]
/// *This icon requires the feature* `TbX` *to be enabled*.
#[component]
pub fn X(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-x" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M18 6l-12 12" /><path d="M6 6l12 12" /></svg>
   }
}