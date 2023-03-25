#[cfg(feature = "TiMediaEject")]
use leptos::*;
#[cfg(feature = "TiMediaEject")]
///This icon requires the feature `TiMediaEject` to be enabled.
#[component]
pub fn MediaEject(
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
        = "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 16h-10c-1.104 0-2 .895-2 2 0 1.104.896 2 2 2h10c1.104 0 2-.896 2-2 0-1.105-.896-2-2-2zM18.433 10.604c-2.574-2.641-6.433-6.604-6.433-6.604s-3.859 3.963-6.433 6.604c-.349.363-.567.853-.567 1.396 0 1.104.896 2 2 2h10c1.104 0 2-.896 2-2 0-.543-.218-1.033-.567-1.396z"
        /> < title > { title } < / title > < / svg >
    }
}
