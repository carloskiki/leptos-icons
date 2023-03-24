#[cfg(feature = "TbPennant2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbPennant2")]
/// *This icon requires the feature* `TbPennant2` *to be enabled*.
#[component]
pub fn Pennant2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-pennant-2" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M16 21h-4" /><path d="M14 21v-18" /><path d="M14 4l-9 4l9 4" /></svg>
   }
}