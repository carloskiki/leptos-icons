#[cfg(feature = "TbArrowWaveLeftDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowWaveLeftDown")]
/// *This icon requires the feature* `TbArrowWaveLeftDown` *to be enabled*.
#[component]
pub fn ArrowWaveLeftDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-wave-left-down" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M7 14h-4v-4" /><path d="M21 12c-.887 1.284 -2.48 2.033 -4 2c-1.52 .033 -3.113 -.716 -4 -2s-2.48 -2.033 -4 -2c-1.52 -.033 -3 1 -4 2l-2 2" /></svg>
   }
}