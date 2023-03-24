#[cfg(feature = "TbCirclesRelation")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbCirclesRelation")]
/// *This icon requires the feature* `TbCirclesRelation` *to be enabled*.
#[component]
pub fn CirclesRelation(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-circles-relation" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M9.183 6.117a6 6 0 1 0 4.511 3.986" /><path d="M14.813 17.883a6 6 0 1 0 -4.496 -3.954" /></svg>
   }
}