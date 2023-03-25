#[cfg(feature = "TiMediaRewindOutline")]
use leptos::*;
#[cfg(feature = "TiMediaRewindOutline")]
///This icon requires the feature `TiMediaRewindOutline` to be enabled.
#[component]
pub fn MediaRewindOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = size.clone() height = size xmlns
        = "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" >< path
        d =
        "M10 8.676v8.05l-4.133-4.025 4.133-4.025m.2-2.276c-.488 0-.931.197-1.253.512-2.381 2.315-5.947 5.789-5.947 5.789l5.944 5.789c.326.315.768.51 1.256.51.994 0 1.8-.805 1.8-1.799v-9c0-.994-.806-1.801-1.8-1.801zM19 8.676v8.051l-4.133-4.025 4.133-4.026m.2-2.276c-.488 0-.931.197-1.253.512-2.381 2.315-5.947 5.789-5.947 5.789l5.944 5.789c.326.315.768.51 1.256.51.994 0 1.8-.805 1.8-1.799v-9c0-.994-.806-1.801-1.8-1.801z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
