#[cfg(feature = "IoTrashSharp")]
use leptos::*;
#[cfg(feature = "IoTrashSharp")]
///This icon requires the feature `IoTrashSharp` to be enabled.
#[component]
pub fn TrashSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M296,64H216a7.91,7.91,0,0,0-8,8V96h96V72A7.91,7.91,0,0,0,296,64Z" style =
        "fill:none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M292,64H220a4,4,0,0,0-4,4V96h80V68A4,4,0,0,0,292,64Z" style = "fill:none" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M447.55,96H336V48a16,16,0,0,0-16-16H192a16,16,0,0,0-16,16V96H64.45L64,136H97l20.09,314A32,32,0,0,0,149,480H363a32,32,0,0,0,31.93-29.95L415,136h33ZM176,416l-9-256h33l9,256Zm96,0H240V160h32ZM296,96H216V68a4,4,0,0,1,4-4h72a4,4,0,0,1,4,4Zm40,320H303l9-256h33Z"
        /> < title > { title } < / title > < / svg >
    }
}
