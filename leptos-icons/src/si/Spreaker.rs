#[cfg(feature = "SiSpreaker")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiSpreaker")]
/// *This icon requires the feature* `SiSpreaker` *to be enabled*.
#[component]
pub fn Spreaker(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M14.615 0l-5.64 6.54L.529 4.718l8.68 7.372-8.537 7.463 8.411-1.984L14.843 24l.71-8.601 7.918-3.483-7.963-3.33L14.621 0h-.006z" /></svg>
   }
}