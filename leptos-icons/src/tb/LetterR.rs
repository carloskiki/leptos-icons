#[cfg(feature = "TbLetterR")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbLetterR")]
/// *This icon requires the feature* `TbLetterR` *to be enabled*.
#[component]
pub fn LetterR(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-letter-r" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M7 20v-16h5.5a4 4 0 0 1 0 9h-5.5" /><path d="M12 13l5 7" /></svg>
   }
}