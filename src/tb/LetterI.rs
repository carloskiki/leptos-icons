#[cfg(feature = "TbLetterI")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbLetterI")]
/// *This icon requires the feature* `TbLetterI` *to be enabled*.
#[component]
pub fn LetterI(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-letter-i" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 4l0 16" /></svg>
   }
}