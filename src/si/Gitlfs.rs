#[cfg(feature = "SiGitlfs")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiGitlfs")]
/// *This icon requires the feature* `SiGitlfs` *to be enabled*.
#[component]
pub fn Gitlfs(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M11.967.304L0 7.215v9.68l11.79 6.802V14.02l11.96-6.91-4.383-2.53-11.959 6.905v3.887l-2.775-1.601V9.886l11.965-6.91zM24 7.545L12.29 14.31v9.387L24 16.929V7.547z" /></svg>
   }
}