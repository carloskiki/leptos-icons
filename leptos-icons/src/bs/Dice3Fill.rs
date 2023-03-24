#[cfg(feature = "BsDice3Fill")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BsDice3Fill")]
/// *This icon requires the feature* `BsDice3Fill` *to be enabled*.
#[component]
pub fn Dice3Fill(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" fill="currentColor" class="bi bi-dice-3-fill" viewBox="0 0 16 16"><path d="M3 0a3 3 0 0 0-3 3v10a3 3 0 0 0 3 3h10a3 3 0 0 0 3-3V3a3 3 0 0 0-3-3H3zm2.5 4a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0zm8 8a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0zM8 9.5a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3z" /></svg>
   }
}