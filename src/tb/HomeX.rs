#[cfg(feature = "TbHomeX")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbHomeX")]
/// *This icon requires the feature* `TbHomeX` *to be enabled*.
#[component]
pub fn HomeX(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-home-x" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M19 13.4v-1.4h2l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h5.5" /><path d="M9 21v-6a2 2 0 0 1 2 -2h2c.402 0 .777 .119 1.091 .323" /><path d="M21.5 21.5l-5 -5" /><path d="M16.5 21.5l5 -5" /></svg>
   }
}