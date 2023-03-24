#[cfg(feature = "IoCard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoCard")]
/// *This icon requires the feature* `IoCard` *to be enabled*.
#[component]
pub fn Card(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><path d="M32,376a56,56,0,0,0,56,56H424a56,56,0,0,0,56-56V222H32Zm66-76a30,30,0,0,1,30-30h48a30,30,0,0,1,30,30v20a30,30,0,0,1-30,30H128a30,30,0,0,1-30-30Z" /><path d="M424,80H88a56,56,0,0,0-56,56v26H480V136A56,56,0,0,0,424,80Z" /></svg>
   }
}