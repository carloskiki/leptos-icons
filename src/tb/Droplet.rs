#[cfg(feature = "TbDroplet")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDroplet")]
/// *This icon requires the feature* `TbDroplet` *to be enabled*.
#[component]
pub fn Droplet(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-droplet" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M6.8 11a6 6 0 1 0 10.396 0l-5.197 -8l-5.2 8z" /></svg>
   }
}