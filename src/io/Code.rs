#[cfg(feature = "IoCode")]
use leptos::*;
#[cfg(feature = "IoCode")]
///This icon requires the feature `IoCode` to be enabled.
#[component]
pub fn Code(
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
        "M160,389a20.91,20.91,0,0,1-13.82-5.2l-128-112a21,21,0,0,1,0-31.6l128-112a21,21,0,0,1,27.66,31.61L63.89,256l109.94,96.19A21,21,0,0,1,160,389Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M352,389a21,21,0,0,1-13.84-36.81L448.11,256,338.17,159.81a21,21,0,0,1,27.66-31.61l128,112a21,21,0,0,1,0,31.6l-128,112A20.89,20.89,0,0,1,352,389Z"
        /> < title > { title } < / title > < / svg >
    }
}
