#[cfg(feature = "VsBookmark")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsBookmark")]
/// *This icon requires the feature* `VsBookmark` *to be enabled*.
#[component]
pub fn Bookmark(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M12.5 1h-9l-.5.5v13l.872.335L8 10.247l4.128 4.588L13 14.5v-13l-.5-.5zM12 13.2L8.372 9.165h-.744L4 13.2V2h8v11.2z" /></svg>
   }
}