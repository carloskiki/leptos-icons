#[cfg(feature = "TbStairsUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbStairsUp")]
/// *This icon requires the feature* `TbStairsUp` *to be enabled*.
#[component]
pub fn StairsUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-stairs-up" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M4 20h4v-4h4v-4h4v-4h4" /><path d="M4 11l7 -7v4m-4 -4h4" /></svg>
   }
}