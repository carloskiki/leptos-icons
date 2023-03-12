#[cfg(feature = "TbCircleArrowUpRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCircleArrowUpRight")]
/// *This icon requires the feature* `TbCircleArrowUpRight` *to be enabled*.
#[component]
pub fn CircleArrowUpRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-circle-arrow-up-right" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 12a9 9 0 1 0 18 0a9 9 0 0 0 -18 0" /><path d="M15 9l-6 6" /><path d="M15 15v-6h-6" /></svg>
   }
}