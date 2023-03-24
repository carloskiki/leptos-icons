#[cfg(feature = "TiThLarge")]
use leptos::*;
#[cfg(feature = "TiThLarge")]
///This icon requires the feature `TiThLarge` to be enabled.
#[component]
pub fn ThLarge(
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
        "M8 3h-2c-.825 0-1.575.337-2.119.881-.544.544-.881 1.294-.881 2.119v2c0 .825.337 1.575.881 2.119.544.544 1.294.881 2.119.881h2c.825 0 1.575-.337 2.119-.881.544-.544.881-1.294.881-2.119v-2c0-.825-.337-1.575-.881-2.119-.544-.544-1.294-.881-2.119-.881zM18 3h-2c-.825 0-1.575.337-2.119.881-.544.544-.881 1.294-.881 2.119v2c0 .825.337 1.575.881 2.119.544.544 1.294.881 2.119.881h2c.825 0 1.575-.337 2.119-.881.544-.544.881-1.294.881-2.119v-2c0-.825-.337-1.575-.881-2.119-.544-.544-1.294-.881-2.119-.881zM8 13h-2c-.825 0-1.575.337-2.119.881-.544.544-.881 1.294-.881 2.119v2c0 .825.337 1.575.881 2.119.544.544 1.294.881 2.119.881h2c.825 0 1.575-.337 2.119-.881.544-.544.881-1.294.881-2.119v-2c0-.825-.337-1.575-.881-2.119-.544-.544-1.294-.881-2.119-.881zM18 13h-2c-.825 0-1.575.337-2.119.881-.544.544-.881 1.294-.881 2.119v2c0 .825.337 1.575.881 2.119.544.544 1.294.881 2.119.881h2c.825 0 1.575-.337 2.119-.881.544-.544.881-1.294.881-2.119v-2c0-.825-.337-1.575-.881-2.119-.544-.544-1.294-.881-2.119-.881z"
        /> < title > { title } < / title > < / svg >
    }
}
