#[cfg(feature = "AiOutlinedSubnode")]
use leptos::*;
#[cfg(feature = "AiOutlinedSubnode")]
///This icon requires the feature `AiOutlinedSubnode` to be enabled.
#[component]
pub fn Subnode(
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
        stroke_witdh = "0" style = style t = "1569683432252" class = "icon" viewBox =
        "0 0 1024 1024" version = "1.1" p - id = "10345" width = "200" height = "200"
        width = { size.clone() } height = { size } > < defs xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" ><
        style type = "text/css" /></ defs >< path xmlns = "http://www.w3.org/2000/svg"
        xmlns : xlink = "http://www.w3.org/1999/xlink" d =
        "M688 240c-138 0-252 102.8-269.6 236H249c-14.2-35.2-48.7-60-89-60-53 0-96 43-96 96s43 96 96 96c40.3 0 74.8-24.8 89-60h169.3C436 681.2 550 784 688 784c150.2 0 272-121.8 272-272S838.2 240 688 240z m128 298c0 4.4-3.6 8-8 8h-86v86c0 4.4-3.6 8-8 8h-52c-4.4 0-8-3.6-8-8v-86h-86c-4.4 0-8-3.6-8-8v-52c0-4.4 3.6-8 8-8h86v-86c0-4.4 3.6-8 8-8h52c4.4 0 8 3.6 8 8v86h86c4.4 0 8 3.6 8 8v52z"
        p - id = "10346" /> < title > { title } < / title > < / svg >
    }
}
