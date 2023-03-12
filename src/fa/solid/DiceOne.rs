#[cfg(feature = "FaSolidDiceOne")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "FaSolidDiceOne")]
/// *This icon requires the feature* `FaSolidDiceOne` *to be enabled*.
#[component]
pub fn DiceOne(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path d="M64 32C28.7 32 0 60.7 0 96V416c0 35.3 28.7 64 64 64H384c35.3 0 64-28.7 64-64V96c0-35.3-28.7-64-64-64H64zM224 288c-17.7 0-32-14.3-32-32s14.3-32 32-32s32 14.3 32 32s-14.3 32-32 32z" /></svg>
   }
}