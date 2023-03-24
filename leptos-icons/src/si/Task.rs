#[cfg(feature = "SiTask")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiTask")]
/// *This icon requires the feature* `SiTask` *to be enabled*.
#[component]
pub fn Task(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M1.857 18.013 11.736 24V12.456L1.857 6.468Zm20.286 0V6.468l-9.879 5.988V24Zm-.246-12.014L12 0 2.103 5.999 12 11.998Z" /></svg>
   }
}