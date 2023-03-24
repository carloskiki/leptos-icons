#[cfg(feature = "TbActivity")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbActivity")]
/// *This icon requires the feature* `TbActivity` *to be enabled*.
#[component]
pub fn Activity(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-activity" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 12h4l3 8l4 -16l3 8h4" /></svg>
   }
}