#[cfg(feature = "SiMitsubishi")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiMitsubishi")]
/// *This icon requires the feature* `SiMitsubishi` *to be enabled*.
#[component]
pub fn Mitsubishi(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M8 22.38H0l4-6.92h8zm8 0h8l-4-6.92h-8zm0-13.84l-4-6.92-4 6.92 4 6.92Z" /></svg>
   }
}