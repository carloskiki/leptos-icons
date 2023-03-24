#[cfg(feature = "RiUserFillEmotionSad")]
use leptos::*;
#[cfg(feature = "RiUserFillEmotionSad")]
///This icon requires the feature `RiUserFillEmotionSad` to be enabled.
#[component]
pub fn EmotionSad(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M12 2c5.523 0 10 4.477 10 10a9.958 9.958 0 0 1-1.065 4.496 1.977 1.977 0 0 0-.398-.775l-.123-.135L19 14.172l-1.414 1.414-.117.127a2 2 0 0 0 1.679 3.282A9.974 9.974 0 0 1 12 22C6.477 22 2 17.523 2 12S6.477 2 12 2zm0 13c-1.38 0-2.63.56-3.534 1.463l-.166.174.945.86C10.035 17.182 10.982 17 12 17c.905 0 1.754.144 2.486.396l.269.1.945-.86A4.987 4.987 0 0 0 12 15zm-3.5-5a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3zm7 0a1.5 1.5 0 1 0 0 3 1.5 1.5 0 0 0 0-3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
