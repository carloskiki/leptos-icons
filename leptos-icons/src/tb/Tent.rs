#[cfg(feature = "TbTent")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbTent")]
/// *This icon requires the feature* `TbTent` *to be enabled*.
#[component]
pub fn Tent(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-tent" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M11 14l4 6h6l-9 -16l-9 16h6l4 -6" /></svg>
   }
}