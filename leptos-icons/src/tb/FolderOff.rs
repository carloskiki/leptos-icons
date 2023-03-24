#[cfg(feature = "TbFolderOff")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbFolderOff")]
/// *This icon requires the feature* `TbFolderOff` *to be enabled*.
#[component]
pub fn FolderOff(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-folder-off" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M8 4h1l3 3h7a2 2 0 0 1 2 2v8m-2 2h-14a2 2 0 0 1 -2 -2v-11a2 2 0 0 1 1.189 -1.829" /><path d="M3 3l18 18" /></svg>
   }
}