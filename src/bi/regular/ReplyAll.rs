#[cfg(feature = "BiRegularReplyAll")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularReplyAll")]
/// *This icon requires the feature* `BiRegularReplyAll` *to be enabled*.
#[component]
pub fn ReplyAll(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 18v-8a1 1 0 0 0-1-1h-6V6l-5 4 5 4v-3h5v7h2z" /><path d="M9 12.4 6 10l3-2.4V6l-5 4 5 4z" /></svg>
   }
}