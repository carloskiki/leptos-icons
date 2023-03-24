#[cfg(feature = "AiFilledRobot")]
use leptos::*;
#[cfg(feature = "AiFilledRobot")]
///This icon requires the feature `AiFilledRobot` to be enabled.
#[component]
pub fn Robot(
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
        stroke_witdh = "0" style = style t = "1569682613478" class = "icon" viewBox =
        "0 0 1024 1024" version = "1.1" p - id = "7948" width = "200" height = "200"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < defs
        xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" >< style type = "text/css" /></ defs >< path xmlns
        = "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" d =
        "M852 64H172c-17.7 0-32 14.3-32 32v660c0 17.7 14.3 32 32 32h680c17.7 0 32-14.3 32-32V96c0-17.7-14.3-32-32-32zM300 328c0-33.1 26.9-60 60-60s60 26.9 60 60-26.9 60-60 60-60-26.9-60-60z m372 248c0 4.4-3.6 8-8 8H360c-4.4 0-8-3.6-8-8v-60c0-4.4 3.6-8 8-8h304c4.4 0 8 3.6 8 8v60z m-8-188c-33.1 0-60-26.9-60-60s26.9-60 60-60 60 26.9 60 60-26.9 60-60 60zM799 864H225c-13.8 0-25 14.3-25 32v56c0 4.4 2.8 8 6.2 8h611.5c3.4 0 6.2-3.6 6.2-8v-56c0.1-17.7-11.1-32-24.9-32z"
        p - id = "7949" /> < title > { title } < / title > < / svg >
    }
}
