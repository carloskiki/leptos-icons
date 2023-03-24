#[cfg(feature = "AiTwotoneSwitcher")]
use leptos::*;
#[cfg(feature = "AiTwotoneSwitcher")]
///This icon requires the feature `AiTwotoneSwitcher` to be enabled.
#[component]
pub fn Switcher(
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
        stroke_witdh = "0" style = style viewBox = "0 0 1024 1024" width = { size.clone()
        } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" fill =
        "#D9D9D9" d = "M184 840h528V312H184v528zm116-290h296v64H300v-64z" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M880 112H264c-4.4 0-8 3.6-8 8v56c0 4.4 3.6 8 8 8h576v576c0 4.4 3.6 8 8 8h56c4.4 0 8-3.6 8-8V144c0-17.7-14.3-32-32-32z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M752 240H144c-17.7 0-32 14.3-32 32v608c0 17.7 14.3 32 32 32h608c17.7 0 32-14.3 32-32V272c0-17.7-14.3-32-32-32zm-40 600H184V312h528v528z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M300 550h296v64H300z" /> <
        title > { title } < / title > < / svg >
    }
}
