#[cfg(feature = "TbBadge")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBadge")]
/// *This icon requires the feature* `TbBadge` *to be enabled*.
#[component]
pub fn Badge(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-badge" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M17 17v-13l-5 3l-5 -3v13l5 3z" /></svg>
   }
}