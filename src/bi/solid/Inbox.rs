#[cfg(feature = "BiSolidInbox")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidInbox")]
/// *This icon requires the feature* `BiSolidInbox` *to be enabled*.
#[component]
pub fn Inbox(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 3H4c-1.103 0-2 .897-2 2v14a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2V5c0-1.103-.897-2-2-2zm-1 9h-3.142c-.446 1.722-1.997 3-3.858 3s-3.412-1.278-3.858-3H4V5h16v7h-1z" /></svg>
   }
}