#[cfg(feature = "SiSpreadshirt")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "SiSpreadshirt")]
/// *This icon requires the feature* `SiSpreadshirt` *to be enabled*.
#[component]
pub fn Spreadshirt(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" role="img" viewBox="0 0 24 24"><path d="M12 6.306L7.796 2.102 0 9.898l12 12 12-12-7.796-7.796zm0 12L3.592 9.898l4.204-4.204L12 9.898l4.184-4.184 4.204 4.204" /></svg>
   }
}