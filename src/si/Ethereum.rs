#[cfg(feature = "SiEthereum")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiEthereum")]
/// *This icon requires the feature* `SiEthereum` *to be enabled*.
#[component]
pub fn Ethereum(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M11.944 17.97L4.58 13.62 11.943 24l7.37-10.38-7.372 4.35h.003zM12.056 0L4.69 12.223l7.365 4.354 7.365-4.35L12.056 0z" /></svg>
   }
}