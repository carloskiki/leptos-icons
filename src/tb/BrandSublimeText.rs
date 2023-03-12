#[cfg(feature = "TbBrandSublimeText")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBrandSublimeText")]
/// *This icon requires the feature* `TbBrandSublimeText` *to be enabled*.
#[component]
pub fn BrandSublimeText(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-brand-sublime-text" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M19 8l-14 4.5v-5.5l14 -4.5z" /><path d="M19 17l-14 4.5v-5.5l14 -4.5z" /><path d="M19 11.5l-14 -4.5" /><path d="M5 12.5l14 4.5" /></svg>
   }
}