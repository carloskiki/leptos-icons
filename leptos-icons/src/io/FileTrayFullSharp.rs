#[cfg(feature = "IoFileTrayFullSharp")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "IoFileTrayFullSharp")]
/// *This icon requires the feature* `IoFileTrayFullSharp` *to be enabled*.
#[component]
pub fn FileTrayFullSharp(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="512" height="512" viewBox="0 0 512 512"><rect x="128" y="128" width="256" height="38" /><rect x="112" y="192" width="288" height="38" /><path d="M448,64H64L32,256V448H480V256ZM436,256H320a64,64,0,0,1-128,0H76L98,106H414Z" /></svg>
   }
}