#[cfg(feature = "SiCrystal")]
use leptos::*;
#[cfg(feature = "SiCrystal")]
///This icon requires the feature `SiCrystal` to be enabled.
#[component]
pub fn Crystal(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M23.964 15.266l-8.687 8.669c-.034.035-.086.052-.121.035L3.29 20.79c-.052-.017-.087-.052-.087-.086L.007 8.856c-.018-.053 0-.087.035-.122L8.728.065c.035-.035.087-.052.121-.035l11.866 3.18c.052.017.087.052.087.086l3.18 11.848c.034.053.016.087-.018.122zm-11.64-9.433L.667 8.943c-.017 0-.035.034-.017.052l8.53 8.512c.017.017.052.017.052-.017l3.127-11.64c.017 0-.018-.035-.035-.017Z"
        /> < title > { title } < / title > < / svg >
    }
}
