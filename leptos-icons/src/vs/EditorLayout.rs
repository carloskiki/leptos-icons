#[cfg(feature = "VsEditorLayout")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "VsEditorLayout")]
/// *This icon requires the feature* `VsEditorLayout` *to be enabled*.
#[component]
pub fn EditorLayout(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16" fill="currentColor"><path d="M15 6.5l-.47-.5H7V1.47L6.53 1H1.47L1 1.47v8.06l.47.47H4v4.53l.47.47h10.06l.47-.47V6.5zM2 9V3h4v6H2zm12 5H5v-4h1.53L7 9.53V8.013h7V14z" /></svg>
   }
}