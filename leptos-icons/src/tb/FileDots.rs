#[cfg(feature = "TbFileDots")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbFileDots")]
/// *This icon requires the feature* `TbFileDots` *to be enabled*.
#[component]
pub fn FileDots(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-file-dots" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M14 3v4a1 1 0 0 0 1 1h4" /><path d="M17 21h-10a2 2 0 0 1 -2 -2v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2z" /><path d="M9 14v.01" /><path d="M12 14v.01" /><path d="M15 14v.01" /></svg>
   }
}