#[cfg(feature = "TbArrowBadgeUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowBadgeUp")]
/// *This icon requires the feature* `TbArrowBadgeUp` *to be enabled*.
#[component]
pub fn ArrowBadgeUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-badge-up" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M17 11v6l-5 -4l-5 4v-6l5 -4z" /></svg>
   }
}