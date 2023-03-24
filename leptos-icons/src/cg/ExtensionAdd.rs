#[cfg(feature = "CgExtensionAdd")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgExtensionAdd")]
/// *This icon requires the feature* `CgExtensionAdd` *to be enabled*.
#[component]
pub fn ExtensionAdd(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M16 4H18V6H20V8H18V10H16V8H14V6H16V4Z" fill="currentColor" /><path fill-rule="evenodd" clip-rule="evenodd" d="M12 12V6H4V20H18V12H12ZM6 8H10V12H6V8ZM10 14V18H6V14H10ZM16 14V18H12V14H16Z" fill="currentColor" /></svg>
   }
}