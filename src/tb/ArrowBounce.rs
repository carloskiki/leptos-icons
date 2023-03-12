#[cfg(feature = "TbArrowBounce")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowBounce")]
/// *This icon requires the feature* `TbArrowBounce` *to be enabled*.
#[component]
pub fn ArrowBounce(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-bounce" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M10 18h4" /><path d="M3 8a9 9 0 0 1 9 9v1l1.428 -4.285a12 12 0 0 1 6.018 -6.938l.554 -.277" /><path d="M15 6h5v5" /></svg>
   }
}