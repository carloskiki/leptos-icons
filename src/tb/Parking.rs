#[cfg(feature = "TbParking")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbParking")]
/// *This icon requires the feature* `TbParking` *to be enabled*.
#[component]
pub fn Parking(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-parking" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 3m0 2a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2z" /><path d="M9 16v-8h4a2 2 0 0 1 0 4h-4" /></svg>
   }
}