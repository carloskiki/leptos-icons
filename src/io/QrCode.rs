#[cfg(feature = "IoQrCode")]
use leptos::*;
#[cfg(feature = "IoQrCode")]
///This icon requires the feature `IoQrCode` to be enabled.
#[component]
pub fn QrCode(
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
        "http://www.w3.org/2000/svg" > < rect xmlns = "http://www.w3.org/2000/svg" x =
        "336" y = "336" width = "80" height = "80" rx = "8" ry = "8" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "272" y = "272" width = "64" height = "64" rx =
        "8" ry = "8" />< rect xmlns = "http://www.w3.org/2000/svg" x = "416" y = "416"
        width = "64" height = "64" rx = "8" ry = "8" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "432" y = "272" width = "48" height = "48" rx =
        "8" ry = "8" />< rect xmlns = "http://www.w3.org/2000/svg" x = "272" y = "432"
        width = "48" height = "48" rx = "8" ry = "8" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M448,32H304a32,32,0,0,0-32,32V208a32,32,0,0,0,32,32H448a32,32,0,0,0,32-32V64A32,32,0,0,0,448,32ZM416,168a8,8,0,0,1-8,8H344a8,8,0,0,1-8-8V104a8,8,0,0,1,8-8h64a8,8,0,0,1,8,8Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M208,32H64A32,32,0,0,0,32,64V208a32,32,0,0,0,32,32H208a32,32,0,0,0,32-32V64A32,32,0,0,0,208,32ZM176,168a8,8,0,0,1-8,8H104a8,8,0,0,1-8-8V104a8,8,0,0,1,8-8h64a8,8,0,0,1,8,8Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M208,272H64a32,32,0,0,0-32,32V448a32,32,0,0,0,32,32H208a32,32,0,0,0,32-32V304A32,32,0,0,0,208,272ZM176,408a8,8,0,0,1-8,8H104a8,8,0,0,1-8-8V344a8,8,0,0,1,8-8h64a8,8,0,0,1,8,8Z"
        /> < title > { title } < / title > < / svg >
    }
}
