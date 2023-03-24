#[cfg(feature = "TbLetterM")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbLetterM")]
/// *This icon requires the feature* `TbLetterM` *to be enabled*.
#[component]
pub fn LetterM(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-letter-m" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M6 20v-16l6 14l6 -14v16" /></svg>
   }
}