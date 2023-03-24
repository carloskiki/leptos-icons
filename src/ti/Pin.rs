#[cfg(feature = "TiPin")]
use leptos::*;
#[cfg(feature = "TiPin")]
///This icon requires the feature `TiPin` to be enabled.
#[component]
pub fn Pin(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.729 4.271c-.389-.391-1.021-.393-1.414-.004-.104.104-.176.227-.225.355-.832 1.736-1.748 2.715-2.904 3.293-1.297.64-2.786 1.085-5.186 1.085-.13 0-.26.025-.382.076-.245.102-.439.297-.541.541-.101.244-.101.52 0 .764.051.123.124.234.217.326l3.243 3.243-4.537 6.05 6.05-4.537 3.242 3.242c.092.094.203.166.326.217.122.051.252.078.382.078s.26-.027.382-.078c.245-.102.44-.295.541-.541.051-.121.077-.252.077-.381 0-2.4.444-3.889 1.083-5.166.577-1.156 1.556-2.072 3.293-2.904.129-.049.251-.121.354-.225.389-.393.387-1.025-.004-1.414l-3.997-4.02z"
        /> < title > { title } < / title > < / svg >
    }
}
