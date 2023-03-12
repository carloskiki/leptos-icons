#[cfg(feature = "TbCircleChevronsRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCircleChevronsRight")]
/// *This icon requires the feature* `TbCircleChevronsRight` *to be enabled*.
#[component]
pub fn CircleChevronsRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-circle-chevrons-right" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M9 9l3 3l-3 3" /><path d="M13 9l3 3l-3 3" /><path d="M3 12a9 9 0 1 0 0 -.265l0 .265z" /></svg>
   }
}