#[cfg(feature = "CgPushRight")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPushRight")]
/// *This icon requires the feature* `CgPushRight` *to be enabled*.
#[component]
pub fn PushRight(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M1 12.9999V10.9999H15.4853L12.2427 7.75724L13.6569 6.34303L19.3137 11.9999L13.6569 17.6567L12.2427 16.2425L15.4853 12.9999H1Z" fill="currentColor" /><path d="M20.2877 6V18H22.2877V6H20.2877Z" fill="currentColor" /></svg>
   }
}