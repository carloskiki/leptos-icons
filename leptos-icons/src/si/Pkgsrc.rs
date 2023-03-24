#[cfg(feature = "SiPkgsrc")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiPkgsrc")]
/// *This icon requires the feature* `SiPkgsrc` *to be enabled*.
#[component]
pub fn Pkgsrc(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="m12.908 8.763 9.157-5.132L11.25 0 1.62 4.42Zm1.5 2.29 9-5.368-.948 11.84-8.191 6.382zM.593 6.712 1.619 18.79 11.922 24l-.12-12.788Z" /></svg>
   }
}