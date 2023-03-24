#[cfg(feature = "IoThumbsUpSharp")]
use leptos::*;
#[cfg(feature = "IoThumbsUpSharp")]
///This icon requires the feature `IoThumbsUpSharp` to be enabled.
#[component]
pub fn ThumbsUpSharp(
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
        "M456,192,300,180l23-89.4C329,64,322.22,48.73,300.53,42l-34.69-9.85a4,4,0,0,0-4.4,1.72l-129,202.34a8,8,0,0,1-6.81,3.81H16V448H133.61a48,48,0,0,1,15.18,2.46l76.3,25.43a80,80,0,0,0,25.3,4.11H428.32c19,0,31.5-13.52,35.23-32.16L496,305.58V232C496,209.94,478,194,456,192Z"
        /> < title > { title } < / title > < / svg >
    }
}
