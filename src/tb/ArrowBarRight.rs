#[cfg(feature = "TbArrowBarRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowBarRight")]
/// *This icon requires the feature* `TbArrowBarRight` *to be enabled*.
#[component]
pub fn ArrowBarRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-bar-right" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M20 12l-10 0" /><path d="M20 12l-4 4" /><path d="M20 12l-4 -4" /><path d="M4 4l0 16" /></svg>
   }
}