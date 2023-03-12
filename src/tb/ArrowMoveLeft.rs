#[cfg(feature = "TbArrowMoveLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowMoveLeft")]
/// *This icon requires the feature* `TbArrowMoveLeft` *to be enabled*.
#[component]
pub fn ArrowMoveLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-move-left" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M13 12h-10" /><path d="M6 15l-3 -3l3 -3" /><path d="M17 12a2 2 0 1 1 4 0a2 2 0 0 1 -4 0z" /></svg>
   }
}