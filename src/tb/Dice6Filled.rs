#[cfg(feature = "TbDice6Filled")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDice6Filled")]
/// *This icon requires the feature* `TbDice6Filled` *to be enabled*.
#[component]
pub fn Dice6Filled(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-dice-6-filled" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M18 3a3 3 0 0 1 3 3v12a3 3 0 0 1 -3 3h-12a3 3 0 0 1 -3 -3v-12a3 3 0 0 1 3 -3zm-2.5 12a1.5 1.5 0 1 0 0 3a1.5 1.5 0 0 0 0 -3zm-7 0a1.5 1.5 0 1 0 0 3a1.5 1.5 0 0 0 0 -3zm0 -4.5a1.5 1.5 0 1 0 0 3a1.5 1.5 0 0 0 0 -3zm7 0a1.5 1.5 0 1 0 0 3a1.5 1.5 0 0 0 0 -3zm-7 -4.5a1.5 1.5 0 1 0 0 3a1.5 1.5 0 0 0 0 -3zm7 0a1.5 1.5 0 1 0 0 3a1.5 1.5 0 0 0 0 -3z" stroke-width="0" fill="currentColor" /></svg>
   }
}