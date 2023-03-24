#[cfg(feature = "TiSocialVimeoCircular")]
use leptos::*;
#[cfg(feature = "TiSocialVimeoCircular")]
///This icon requires the feature `TiSocialVimeoCircular` to be enabled.
#[component]
pub fn SocialVimeoCircular(
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
        "M14.463 9.141c-.512 0-.988.238-1.43.715-.311.312-.5.678-.566 1.101.207-.131.387-.196.541-.196.159 0 .29.07.393.212.199.278.188.629-.033 1.051-.489.934-.846 1.4-1.066 1.4-.156 0-.356-.522-.602-1.567-.043-.155-.088-.378-.133-.667-.045-.288-.088-.538-.133-.75-.045-.211-.111-.428-.2-.649s-.2-.384-.333-.483c-.094-.069-.202-.104-.327-.104l-.173.021c-.289.067-.623.245-1 .534-.379.288-.689.566-.934.833l-.334.4.301.399.166-.133c.066-.045.189-.101.367-.167l.191-.043c.069 0 .116.025.143.076.066.089.271.717.615 1.884.346 1.167.562 1.839.65 2.017.156.311.367.55.633.717.13.08.271.121.427.121.165 0 .346-.047.54-.139.601-.399 1.289-1.066 2.067-2 .778-.933 1.255-1.922 1.433-2.966.156-.911-.11-1.434-.799-1.567-.138-.035-.272-.05-.404-.05zM12 21c-4.963 0-9-4.037-9-9s4.037-9 9-9 9 4.037 9 9-4.037 9-9 9zm0-16c-3.859 0-7 3.141-7 7s3.141 7 7 7 7-3.141 7-7-3.141-7-7-7z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
