#[cfg(feature = "TbTrendingUp2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbTrendingUp2")]
/// *This icon requires the feature* `TbTrendingUp2` *to be enabled*.
#[component]
pub fn TrendingUp2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-trending-up-2" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M18 5l3 3l-3 3" /><path d="M3 18h5l7 -10h6" /></svg>
   }
}