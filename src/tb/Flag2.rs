#[cfg(feature = "TbFlag2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbFlag2")]
/// *This icon requires the feature* `TbFlag2` *to be enabled*.
#[component]
pub fn Flag2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-flag-2" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M5 14h14v-9h-14v16" /></svg>
   }
}