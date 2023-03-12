#[cfg(feature = "TbExposureMinus1")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbExposureMinus1")]
/// *This icon requires the feature* `TbExposureMinus1` *to be enabled*.
#[component]
pub fn ExposureMinus1(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-exposure-minus-1" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 12h6" /><path d="M18 19v-14l-4 4" /></svg>
   }
}