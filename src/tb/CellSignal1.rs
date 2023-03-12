#[cfg(feature = "TbCellSignal1")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCellSignal1")]
/// *This icon requires the feature* `TbCellSignal1` *to be enabled*.
#[component]
pub fn CellSignal1(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-cell-signal-1" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M20 20h-15.269a.731 .731 0 0 1 -.517 -1.249l14.537 -14.537a.731 .731 0 0 1 1.249 .517v15.269z" /></svg>
   }
}