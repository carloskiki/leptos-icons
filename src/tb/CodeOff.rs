#[cfg(feature = "TbCodeOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCodeOff")]
/// *This icon requires the feature* `TbCodeOff` *to be enabled*.
#[component]
pub fn CodeOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-code-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M7 8l-4 4l4 4" /><path d="M17 8l4 4l-2.5 2.5" /><path d="M14 4l-1.201 4.805m-.802 3.207l-2 7.988" /><path d="M3 3l18 18" /></svg>
   }
}