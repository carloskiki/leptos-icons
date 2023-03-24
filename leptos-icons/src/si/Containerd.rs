#[cfg(feature = "SiContainerd")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiContainerd")]
/// *This icon requires the feature* `SiContainerd` *to be enabled*.
#[component]
pub fn Containerd(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M3.629 0v24H20.37V0zM17.59 21.208H6.421V10.604h7.812V6.692h3.346v14.516zm-7.823-7.812h4.466v5.02H9.767z" /></svg>
   }
}