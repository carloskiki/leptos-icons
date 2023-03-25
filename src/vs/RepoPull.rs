#[cfg(feature = "VsRepoPull")]
use leptos::*;
#[cfg(feature = "VsRepoPull")]
///This icon requires the feature `VsRepoPull` to be enabled.
#[component]
pub fn RepoPull(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M13 1.5V3h-1V2H3v8h10v3.5l-.5.5H8v-1h4v-2H2.735a.72.72 0 0 0-.285.06.74.74 0 0 0-.4.4.93.93 0 0 0-.05.29v.5a.93.93 0 0 0 .05.29.74.74 0 0 0 .4.4c.091.04.19.06.29.06H3v1h-.26a1.9 1.9 0 0 1-.67-.13 1.77 1.77 0 0 1-.94-.95 1.7 1.7 0 0 1-.13-.67v-9.5a1.7 1.7 0 0 1 .13-.62 1.77 1.77 0 0 1 .94-1A1.9 1.9 0 0 1 2.74 1h9.76l.5.5zM2 10.17V2.748v7.422zM5 3H4v1h1V3zm0 2H4v1h1V5zM4 7h1v1H4V7zm8.07-3.61l-.7.71 1.92 1.92H7v1h6.39l-2.02 2.03.7.7 2.83-2.82v-.71l-2.83-2.83zM5.5 13.49L4.28 15H4v-3h3v3h-.28L5.5 13.49z"
        /> < title > { title } < / title > < / svg >
    }
}
