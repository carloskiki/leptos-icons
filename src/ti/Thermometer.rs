#[cfg(feature = "TiThermometer")]
use leptos::*;
#[cfg(feature = "TiThermometer")]
///This icon requires the feature `TiThermometer` to be enabled.
#[component]
pub fn Thermometer(
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
        = "http://www.w3.org/2000/svg" > < g xmlns = "http://www.w3.org/2000/svg" >< path
        d =
        "M13 15.071v-5.571c0-.275-.225-.5-.5-.5s-.5.225-.5.5v5.571c-.86.224-1.5 1-1.5 1.929 0 1.103.896 2 2 2s2-.897 2-2c0-.929-.64-1.705-1.5-1.929zM16 13.459v-7.959c0-1.93-1.57-3.5-3.5-3.5s-3.5 1.57-3.5 3.5v7.959c-.922.902-1.5 2.151-1.5 3.541 0 2.757 2.243 5 5 5s5-2.243 5-5c0-1.39-.578-2.639-1.5-3.541zm-3.5 6.541c-1.654 0-3-1.346-3-3 0-1.105.607-2.062 1.5-2.583v-8.917c0-.827.673-1.5 1.5-1.5s1.5.673 1.5 1.5v8.917c.893.521 1.5 1.478 1.5 2.583 0 1.654-1.346 3-3 3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
