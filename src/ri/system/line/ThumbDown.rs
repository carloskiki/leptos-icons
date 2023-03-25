#[cfg(feature = "RiSystemLineThumbDown")]
use leptos::*;
#[cfg(feature = "RiSystemLineThumbDown")]
///This icon requires the feature `RiSystemLineThumbDown` to be enabled.
#[component]
pub fn ThumbDown(
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
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M9.4 16H3a2 2 0 0 1-2-2v-2.104a2 2 0 0 1 .15-.762L4.246 3.62A1 1 0 0 1 5.17 3H22a1 1 0 0 1 1 1v10a1 1 0 0 1-1 1h-3.482a1 1 0 0 0-.817.423l-5.453 7.726a.5.5 0 0 1-.632.159L9.802 22.4a2.5 2.5 0 0 1-1.305-2.853L9.4 16zm7.6-2.588V5H5.84L3 11.896V14h6.4a2 2 0 0 1 1.938 2.493l-.903 3.548a.5.5 0 0 0 .261.571l.661.33 4.71-6.672c.25-.354.57-.644.933-.858zM19 13h2V5h-2v8z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
