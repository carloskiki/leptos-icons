#[cfg(feature = "BsDice1")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsDice1")]
/// *This icon requires the feature* `BsDice1` *to be enabled*.
#[component]
pub fn Dice1(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-dice-1" viewBox="0 0 16 16"><circle cx="8" cy="8" r="1.5" /><path d="M13 1a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V3a2 2 0 0 1 2-2h10zM3 0a3 3 0 0 0-3 3v10a3 3 0 0 0 3 3h10a3 3 0 0 0 3-3V3a3 3 0 0 0-3-3H3z" /></svg>
   }
}