#[cfg(feature = "TbFolderBolt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbFolderBolt")]
/// *This icon requires the feature* `TbFolderBolt` *to be enabled*.
#[component]
pub fn FolderBolt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-folder-bolt" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M13 19h-8a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 2 -2h4l3 3h7a2 2 0 0 1 2 2v3.5" /><path d="M19 16l-2 3h4l-2 3" /></svg>
   }
}