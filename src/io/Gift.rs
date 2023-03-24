#[cfg(feature = "IoGift")]
use leptos::*;
#[cfg(feature = "IoGift")]
///This icon requires the feature `IoGift` to be enabled.
#[component]
pub fn Gift(
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
        "M200,144h40V104a40,40,0,1,0-40,40Z" style = "fill:none" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M352,104a40,40,0,0,0-80,0v40h40A40,40,0,0,0,352,104Z" style = "fill:none" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M80,416a64,64,0,0,0,64,64h92a4,4,0,0,0,4-4V292a4,4,0,0,0-4-4H88a8,8,0,0,0-8,8Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M240,252V144h32V252a4,4,0,0,0,4,4H416a47.93,47.93,0,0,0,16-2.75h0A48.09,48.09,0,0,0,464,208V192a48,48,0,0,0-48-48H375.46a2,2,0,0,1-1.7-3A72,72,0,0,0,256,58.82,72,72,0,0,0,138.24,141a2,2,0,0,1-1.7,3H96a48,48,0,0,0-48,48v16a48.09,48.09,0,0,0,32,45.25h0A47.93,47.93,0,0,0,96,256H236A4,4,0,0,0,240,252Zm32-148a40,40,0,1,1,40,40H272ZM197.14,64.1A40,40,0,0,1,240,104v40H200a40,40,0,0,1-2.86-79.89Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M276,480h92a64,64,0,0,0,64-64V296a8,8,0,0,0-8-8H276a4,4,0,0,0-4,4V476A4,4,0,0,0,276,480Z"
        /> < title > { title } < / title > < / svg >
    }
}
