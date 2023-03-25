#[cfg(feature = "RiOthersLineKnifeBlood")]
use leptos::*;
#[cfg(feature = "RiOthersLineKnifeBlood")]
///This icon requires the feature `RiOthersLineKnifeBlood` to be enabled.
#[component]
pub fn KnifeBlood(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path
        fill - rule = "nonzero" d =
        "M4.342 1.408L22.373 19.44a1.5 1.5 0 0 1-2.121 2.122l-4.596-4.597L12.12 20.5 8 16.38V19a1 1 0 0 1-2 0v-4a1 1 0 0 0-1.993-.117L4 15v1a1 1 0 0 1-2 0V7.214a7.976 7.976 0 0 1 2.168-5.627l.174-.179zm.241 3.07l-.051.11a5.993 5.993 0 0 0-.522 2.103L4 7l-.001.12a5.984 5.984 0 0 0 1.58 4.003l.177.185 6.363 6.363 2.829-2.828L4.583 4.478z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
