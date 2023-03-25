#[cfg(feature = "AiTwotoneGift")]
use leptos::*;
#[cfg(feature = "AiTwotoneGift")]
///This icon requires the feature `AiTwotoneGift` to be enabled.
#[component]
pub fn Gift(
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
        stroke_witdh = "0" style = style viewBox = "0 0 1024 1024" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill = "#D9D9D9" d =
        "M546 378h298v104H546zM228 550h250v308H228zm-48-172h298v104H180zm366 172h250v308H546z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M880 310H732.4c13.6-21.4 21.6-46.8 21.6-74 0-76.1-61.9-138-138-138-41.4 0-78.7 18.4-104 47.4-25.3-29-62.6-47.4-104-47.4-76.1 0-138 61.9-138 138 0 27.2 7.9 52.6 21.6 74H144c-17.7 0-32 14.3-32 32v200c0 4.4 3.6 8 8 8h40v344c0 17.7 14.3 32 32 32h640c17.7 0 32-14.3 32-32V550h40c4.4 0 8-3.6 8-8V342c0-17.7-14.3-32-32-32zM478 858H228V550h250v308zm0-376H180V378h298v104zm0-176h-70c-38.6 0-70-31.4-70-70s31.4-70 70-70 70 31.4 70 70v70zm68-70c0-38.6 31.4-70 70-70s70 31.4 70 70-31.4 70-70 70h-70v-70zm250 622H546V550h250v308zm48-376H546V378h298v104z"
        /> < title > { title } < / title > < / svg >
    }
}
