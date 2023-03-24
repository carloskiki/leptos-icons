#[cfg(feature = "TbInfoSmall")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbInfoSmall")]
/// *This icon requires the feature* `TbInfoSmall` *to be enabled*.
#[component]
pub fn InfoSmall(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-info-small" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 9h.01" /><path d="M11 12h1v4h1" /></svg>
   }
}