#[cfg(feature = "SiMicrosoftonenote")]
use leptos::*;
#[cfg(feature = "SiMicrosoftonenote")]
///This icon requires the feature `SiMicrosoftonenote` to be enabled.
#[component]
pub fn Microsoftonenote(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M23 1.5Q23.41 1.5 23.7 1.8 24 2.09 24 2.5V21.5Q24 21.91 23.7 22.2 23.41 22.5 23 22.5H7Q6.59 22.5 6.3 22.2 6 21.91 6 21.5V18H1Q0.59 18 0.3 17.7 0 17.41 0 17V7Q0 6.59 0.3 6.3 0.58 6 1 6H6V2.5Q6 2.09 6.3 1.8 6.59 1.5 7 1.5ZM4.56 11 7.39 15.93H9.18V8.07H7.44V13.1L4.71 8.07H2.82V15.93H4.56ZM22.5 21V18H19.5V21ZM22.5 16.5V13.5H19.5V16.5ZM22.5 12V9H19.5V12ZM22.5 7.5V3H7.5V6H11Q11.41 6 11.7 6.3 12 6.59 12 7V17Q12 17.41 11.7 17.7 11.41 18 11 18H7.5V21H18V7.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
