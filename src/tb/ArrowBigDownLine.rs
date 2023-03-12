#[cfg(feature = "TbArrowBigDownLine")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowBigDownLine")]
/// *This icon requires the feature* `TbArrowBigDownLine` *to be enabled*.
#[component]
pub fn ArrowBigDownLine(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-big-down-line" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M15 12h3.586a1 1 0 0 1 .707 1.707l-6.586 6.586a1 1 0 0 1 -1.414 0l-6.586 -6.586a1 1 0 0 1 .707 -1.707h3.586v-6h6v6z" /><path d="M15 3h-6" /></svg>
   }
}