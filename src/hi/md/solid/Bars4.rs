#[cfg(feature = "HiMdSolidBars4")]
use leptos::*;
#[cfg(feature = "HiMdSolidBars4")]
///This icon requires the feature `HiMdSolidBars4` to be enabled.
#[component]
pub fn Bars4(
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
        "M2 3.75C2 3.33579 2.33579 3 2.75 3H17.25C17.6642 3 18 3.33579 18 3.75C18 4.16421 17.6642 4.5 17.25 4.5H2.75C2.33579 4.5 2 4.16421 2 3.75ZM2 7.91667C2 7.50245 2.33579 7.16667 2.75 7.16667H17.25C17.6642 7.16667 18 7.50245 18 7.91667C18 8.33088 17.6642 8.66667 17.25 8.66667H2.75C2.33579 8.66667 2 8.33088 2 7.91667ZM2 12.0833C2 11.6691 2.33579 11.3333 2.75 11.3333H17.25C17.6642 11.3333 18 11.6691 18 12.0833C18 12.4975 17.6642 12.8333 17.25 12.8333H2.75C2.33579 12.8333 2 12.4975 2 12.0833ZM2 16.25C2 15.8358 2.33579 15.5 2.75 15.5H17.25C17.6642 15.5 18 15.8358 18 16.25C18 16.6642 17.6642 17 17.25 17H2.75C2.33579 17 2 16.6642 2 16.25Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
