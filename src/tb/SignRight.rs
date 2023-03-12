#[cfg(feature = "TbSignRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbSignRight")]
/// *This icon requires the feature* `TbSignRight` *to be enabled*.
#[component]
pub fn SignRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-sign-right" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M8 21h4" /><path d="M10 21v-10" /><path d="M10 6v-3" /><path d="M6 6h10l2 2.5l-2 2.5h-10z" /></svg>
   }
}