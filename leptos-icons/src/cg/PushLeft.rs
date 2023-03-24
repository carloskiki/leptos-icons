#[cfg(feature = "CgPushLeft")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "CgPushLeft")]
/// *This icon requires the feature* `CgPushLeft` *to be enabled*.
#[component]
pub fn PushLeft(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none"><path d="M22.2877 11.0001V13.0001H7.80237L11.045 16.2428L9.63079 17.657L3.97394 12.0001L9.63079 6.34326L11.045 7.75748L7.80236 11.0001H22.2877Z" fill="currentColor" /><path d="M3 18V6H1V18H3Z" fill="currentColor" /></svg>
   }
}