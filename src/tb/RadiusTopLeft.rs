#[cfg(feature = "TbRadiusTopLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbRadiusTopLeft")]
/// *This icon requires the feature* `TbRadiusTopLeft` *to be enabled*.
#[component]
pub fn RadiusTopLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-radius-top-left" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M5 19v-6a8 8 0 0 1 8 -8h6" /></svg>
   }
}