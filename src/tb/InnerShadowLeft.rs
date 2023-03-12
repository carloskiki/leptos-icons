#[cfg(feature = "TbInnerShadowLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbInnerShadowLeft")]
/// *This icon requires the feature* `TbInnerShadowLeft` *to be enabled*.
#[component]
pub fn InnerShadowLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-inner-shadow-left" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M5.636 5.636a9 9 0 1 1 12.728 12.728a9 9 0 0 1 -12.728 -12.728z" /><path d="M7.757 16.243a6 6 0 0 1 0 -8.486" /></svg>
   }
}