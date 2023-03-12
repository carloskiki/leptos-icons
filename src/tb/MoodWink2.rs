#[cfg(feature = "TbMoodWink2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "TbMoodWink2")]
/// *This icon requires the feature* `TbMoodWink2` *to be enabled*.
#[component]
pub fn MoodWink2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" class="icon icon-tabler icon-tabler-mood-wink-2" width="24" height="24" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" fill="none" stroke-linecap="round" stroke-linejoin="round"><path stroke="none" d="M0 0h24v24H0z" fill="none" /><path d="M12 21a9 9 0 1 1 0 -18a9 9 0 0 1 0 18z" /><path d="M9 10h-.01" /><path d="M14.5 15a3.5 3.5 0 0 1 -5 0" /><path d="M15.5 8.5l-1.5 1.5l1.5 1.5" /></svg>
   }
}