#[cfg(feature = "TbDice6")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbDice6")]
/// *This icon requires the feature* `TbDice6` *to be enabled*.
#[component]
pub fn Dice6(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-dice-6" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M3 3m0 2a2 2 0 0 1 2 -2h14a2 2 0 0 1 2 2v14a2 2 0 0 1 -2 2h-14a2 2 0 0 1 -2 -2z" /><circle cx="8.5" cy="7.5" r=".5" fill="currentColor" /><circle cx="15.5" cy="7.5" r=".5" fill="currentColor" /><circle cx="8.5" cy="12" r=".5" fill="currentColor" /><circle cx="15.5" cy="12" r=".5" fill="currentColor" /><circle cx="15.5" cy="16.5" r=".5" fill="currentColor" /><circle cx="8.5" cy="16.5" r=".5" fill="currentColor" /></svg>
   }
}