#[cfg(feature = "TbEqualNot")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbEqualNot")]
/// *This icon requires the feature* `TbEqualNot` *to be enabled*.
#[component]
pub fn EqualNot(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-equal-not" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M5 10h14" /><path d="M5 14h14" /><path d="M5 19l14 -14" /></svg>
   }
}