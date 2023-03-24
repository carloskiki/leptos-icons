#[cfg(feature = "TbArrowLeftBar")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowLeftBar")]
/// *This icon requires the feature* `TbArrowLeftBar` *to be enabled*.
#[component]
pub fn ArrowLeftBar(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-left-bar" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M21 12h-18" /><path d="M6 9l-3 3l3 3" /><path d="M21 9v6" /></svg>
   }
}