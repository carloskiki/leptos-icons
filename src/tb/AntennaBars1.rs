#[cfg(feature = "TbAntennaBars1")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbAntennaBars1")]
/// *This icon requires the feature* `TbAntennaBars1` *to be enabled*.
#[component]
pub fn AntennaBars1(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-antenna-bars-1" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M6 18l0 .01" /><path d="M10 18l0 .01" /><path d="M14 18l0 .01" /><path d="M18 18l0 .01" /></svg>
   }
}