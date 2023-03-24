#[cfg(feature = "TbTrendingUp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbTrendingUp")]
/// *This icon requires the feature* `TbTrendingUp` *to be enabled*.
#[component]
pub fn TrendingUp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-trending-up" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 17l6 -6l4 4l8 -8" /><path d="M14 7l7 0l0 7" /></svg>
   }
}