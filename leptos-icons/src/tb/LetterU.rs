#[cfg(feature = "TbLetterU")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbLetterU")]
/// *This icon requires the feature* `TbLetterU` *to be enabled*.
#[component]
pub fn LetterU(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-letter-u" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M6 4v11a5 5 0 0 0 5 5h2a5 5 0 0 0 5 -5v-11" /></svg>
   }
}