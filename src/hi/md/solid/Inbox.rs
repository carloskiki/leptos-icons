#[cfg(feature = "HiMdSolidInbox")]
use leptos::*;
#[cfg(feature = "HiMdSolidInbox")]
///This icon requires the feature `HiMdSolidInbox` to be enabled.
#[component]
pub fn Inbox(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M1 11.2708C1 11.0237 1.03331 10.7777 1.09902 10.5395L2.622 5.0187C2.95099 3.82611 4.03584 3 5.27298 3H14.727C15.9642 3 17.049 3.8261 17.378 5.0187L18.901 10.5395C18.9667 10.7777 19 11.0237 19 11.2708V15C19 16.1046 18.1046 17 17 17H3C1.89543 17 1 16.1046 1 15V11.2708ZM4.06799 5.41759C4.21753 4.8755 4.71065 4.5 5.27298 4.5H14.727C15.2894 4.5 15.7825 4.8755 15.932 5.41759L17.455 10.9384C17.4606 10.9588 17.4657 10.9794 17.4703 11H14C13.6471 11 13.3203 11.186 13.1401 11.4895L12.5339 12.5105C12.3537 12.814 12.0269 13 11.674 13H8.23607C7.8573 13 7.51103 12.786 7.34164 12.4472L6.89443 11.5528C6.72504 11.214 6.37877 11 6 11H2.52969C2.53427 10.9794 2.53938 10.9588 2.54501 10.9384L4.06799 5.41759Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
