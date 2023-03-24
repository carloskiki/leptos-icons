#[cfg(feature = "FaSolidManatSign")]
use leptos::*;
#[cfg(feature = "FaSolidManatSign")]
///This icon requires the feature `FaSolidManatSign` to be enabled.
#[component]
pub fn ManatSign(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M192 32c-17.7 0-32 14.3-32 32V98.7C69.2 113.9 0 192.9 0 288V448c0 17.7 14.3 32 32 32s32-14.3 32-32V288c0-59.6 40.8-109.8 96-124V448c0 17.7 14.3 32 32 32s32-14.3 32-32V164c55.2 14.2 96 64.3 96 124V448c0 17.7 14.3 32 32 32s32-14.3 32-32V288c0-95.1-69.2-174.1-160-189.3V64c0-17.7-14.3-32-32-32z"
        /> < title > { title } < / title > < / svg >
    }
}
