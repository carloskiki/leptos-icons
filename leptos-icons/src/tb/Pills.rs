#[cfg(feature = "TbPills")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbPills")]
/// *This icon requires the feature* `TbPills` *to be enabled*.
#[component]
pub fn Pills(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-pills" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M8 8m-5 0a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /><path d="M17 17m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" /><path d="M4.5 4.5l7 7" /><path d="M19.5 14.5l-5 5" /></svg>
   }
}