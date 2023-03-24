#[cfg(feature = "TbBrandFlutter")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbBrandFlutter")]
/// *This icon requires the feature* `TbBrandFlutter` *to be enabled*.
#[component]
pub fn BrandFlutter(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-brand-flutter" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M7 14l-3 -3l8 -8h6z" /><path d="M14 21l-5 -5l5 -5h5l-5 5l5 5z" /></svg>
   }
}