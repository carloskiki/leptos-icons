#[cfg(feature = "SiMacys")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiMacys")]
/// *This icon requires the feature* `SiMacys` *to be enabled*.
#[component]
pub fn Macys(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12.015.624L9.19 9.293H0l7.445 5.384-2.819 8.673L12 17.986l7.422 5.393-2.835-8.713L24 9.292h-9.162L12.015.622v.002z" /></svg>
   }
}