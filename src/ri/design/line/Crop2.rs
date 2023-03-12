#[cfg(feature = "RiDesignLineCrop2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiDesignLineCrop2")]
/// *This icon requires the feature* `RiDesignLineCrop2` *to be enabled*.
#[component]
pub fn Crop2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M8.414 17H15v2H6a1 1 0 0 1-1-1V7H2V5h3V2h2v13.586L15.586 7H9V5h8.586l2.556-2.556 1.414 1.414L19 6.414V17h3v2h-3v3h-2V8.414L8.414 17z" /></g></svg>
   }
}