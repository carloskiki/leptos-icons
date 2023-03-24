#[cfg(feature = "RiUserFillGhost2")]
use leptos::*;
#[cfg(feature = "RiUserFillGhost2")]
///This icon requires the feature `RiUserFillGhost2` to be enabled.
#[component]
pub fn Ghost2(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M12 2c3.5 0 6 3 7 6 3 1 4 3.73 4 6l-2.775.793a1 1 0 0 0-.725.961v1.496A1.75 1.75 0 0 1 17.75 19h-.596a2 2 0 0 0-1.668.896C14.558 21.3 13.396 22 12 22c-1.396 0-2.558-.701-3.486-2.104A2 2 0 0 0 6.846 19H6.25a1.75 1.75 0 0 1-1.75-1.75v-1.496a1 1 0 0 0-.725-.961L1 14c0-2.266 1-5 4-6 1-3 3.5-6 7-6zm0 10c-.828 0-1.5 1.12-1.5 2.5S11.172 17 12 17s1.5-1.12 1.5-2.5S12.828 12 12 12zM9.5 8a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3zm5 0a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
