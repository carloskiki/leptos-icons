#[cfg(feature = "TiZoomInOutline")]
use leptos::*;
#[cfg(feature = "TiZoomInOutline")]
///This icon requires the feature `TiZoomInOutline` to be enabled.
#[component]
pub fn ZoomInOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M13 11h-2v-2c0-.275-.225-.5-.5-.5s-.5.225-.5.5v2h-2c-.275 0-.5.225-.5.5s.225.5.5.5h2v2c0 .275.225.5.5.5s.5-.225.5-.5v-2h2c.275 0 .5-.225.5-.5s-.225-.5-.5-.5zM19.381 15.956l-2.244-2.283c.227-.687.363-1.412.363-2.173 0-3.859-3.141-7-7-7s-7 3.141-7 7 3.141 7 7 7c.762 0 1.488-.137 2.173-.364l2.397 2.386c.601.506 1.348.783 2.104.783 1.727 0 3.131-1.404 3.131-3.131 0-.84-.328-1.628-.924-2.218zm-3.901-1.11l2.492 2.531c.205.203.332.486.332.797 0 .625-.507 1.131-1.131 1.131-.312 0-.594-.127-.816-.313l-2.512-2.511c.646-.436 1.201-.991 1.635-1.635zm-9.98-3.346c0-2.757 2.243-5 5-5s5 2.243 5 5-2.243 5-5 5-5-2.243-5-5z"
        /> < title > { title } < / title > < / svg >
    }
}
