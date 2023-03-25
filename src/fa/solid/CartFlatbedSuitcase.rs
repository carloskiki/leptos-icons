#[cfg(feature = "FaSolidCartFlatbedSuitcase")]
use leptos::*;
#[cfg(feature = "FaSolidCartFlatbedSuitcase")]
///This icon requires the feature `FaSolidCartFlatbedSuitcase` to be enabled.
#[component]
pub fn CartFlatbedSuitcase(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M0 32C0 14.3 14.3 0 32 0H48c44.2 0 80 35.8 80 80V368c0 8.8 7.2 16 16 16H608c17.7 0 32 14.3 32 32s-14.3 32-32 32H541.3c1.8 5 2.7 10.4 2.7 16c0 26.5-21.5 48-48 48s-48-21.5-48-48c0-5.6 1-11 2.7-16H253.3c1.8 5 2.7 10.4 2.7 16c0 26.5-21.5 48-48 48s-48-21.5-48-48c0-5.6 1-11 2.7-16H144c-44.2 0-80-35.8-80-80V80c0-8.8-7.2-16-16-16H32C14.3 64 0 49.7 0 32zM432 96V56c0-4.4-3.6-8-8-8H344c-4.4 0-8 3.6-8 8V96h96zM288 96V56c0-30.9 25.1-56 56-56h80c30.9 0 56 25.1 56 56V96 320H288V96zM512 320V96h16c26.5 0 48 21.5 48 48V272c0 26.5-21.5 48-48 48H512zM240 96h16V320H240c-26.5 0-48-21.5-48-48V144c0-26.5 21.5-48 48-48z"
        /> < title > { title } < / title > < / svg >
    }
}
