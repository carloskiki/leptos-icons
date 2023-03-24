#[cfg(feature = "TbArrowElbowRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbArrowElbowRight")]
/// *This icon requires the feature* `TbArrowElbowRight` *to be enabled*.
#[component]
pub fn ArrowElbowRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-arrow-elbow-right" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M21 14v-6h-6" /><path d="M21 8l-9 9l-9 -9" /></svg>
   }
}