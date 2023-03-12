#[cfg(feature = "BiSolidMemoryCard")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidMemoryCard")]
/// *This icon requires the feature* `BiSolidMemoryCard` *to be enabled*.
#[component]
pub fn MemoryCard(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M15 2H6c-1.103 0-2 .897-2 2v16c0 1.103.897 2 2 2h12c1.103 0 2-.897 2-2V7l-5-5zm-6 8H7V6h2v4zm3 0h-2V6h2v4zm3 0h-2V6h2v4z" /></svg>
   }
}