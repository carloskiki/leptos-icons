#[cfg(feature = "FaSolidToiletsPortable")]
use leptos::*;
#[cfg(feature = "FaSolidToiletsPortable")]
///This icon requires the feature `FaSolidToiletsPortable` to be enabled.
#[component]
pub fn ToiletsPortable(
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
        stroke_witdh = "0" style = style viewBox = "0 0 576 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M32 0H224c17.7 0 32 14.3 32 32V64H0V32C0 14.3 14.3 0 32 0zM0 96H24 232h24v24V488c0 13.3-10.7 24-24 24s-24-10.7-24-24v-8H48v8c0 13.3-10.7 24-24 24s-24-10.7-24-24V120 96zM192 224c-8.8 0-16 7.2-16 16v64c0 8.8 7.2 16 16 16s16-7.2 16-16V240c0-8.8-7.2-16-16-16zM352 0H544c17.7 0 32 14.3 32 32V64H320V32c0-17.7 14.3-32 32-32zM320 96h24H552h24v24V488c0 13.3-10.7 24-24 24s-24-10.7-24-24v-8H368v8c0 13.3-10.7 24-24 24s-24-10.7-24-24V120 96zM512 224c-8.8 0-16 7.2-16 16v64c0 8.8 7.2 16 16 16s16-7.2 16-16V240c0-8.8-7.2-16-16-16z"
        /> < title > { title } < / title > < / svg >
    }
}
