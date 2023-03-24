#[cfg(feature = "SiProsieben")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiProsieben")]
/// *This icon requires the feature* `SiProsieben` *to be enabled*.
#[component]
pub fn Prosieben(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M.24 0H23.68V6.64H.24M23.76 7.92V24H.24s2.88-7.84 10.48-12.48c7.12-4.4 13.04-3.6 13.04-3.6Z" /></svg>
   }
}