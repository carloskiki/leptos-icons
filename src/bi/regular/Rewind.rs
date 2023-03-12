#[cfg(feature = "BiRegularRewind")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiRegularRewind")]
/// *This icon requires the feature* `BiRegularRewind` *to be enabled*.
#[component]
pub fn Rewind(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M12 12V7l-7 5 7 5zm7-5-7 5 7 5z" /></svg>
   }
}