#[cfg(feature = "TbArrowsDown")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowsDown")]
/// *This icon requires the feature* `TbArrowsDown` *to be enabled*.
#[component]
pub fn ArrowsDown(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrows-down" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M7 21l0 -18" /><path d="M20 18l-3 3l-3 -3" /><path d="M4 18l3 3l3 -3" /><path d="M17 21l0 -18" /></svg>
   }
}