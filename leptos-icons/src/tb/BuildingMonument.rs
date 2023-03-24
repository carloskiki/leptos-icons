#[cfg(feature = "TbBuildingMonument")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBuildingMonument")]
/// *This icon requires the feature* `TbBuildingMonument` *to be enabled*.
#[component]
pub fn BuildingMonument(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-building-monument" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M8 18l2 -13l2 -2l2 2l2 13" /><path d="M5 21v-3h14v3" /><path d="M3 21l18 0" /></svg>
   }
}