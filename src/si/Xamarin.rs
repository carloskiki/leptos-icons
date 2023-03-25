#[cfg(feature = "SiXamarin")]
use leptos::*;
#[cfg(feature = "SiXamarin")]
///This icon requires the feature `SiXamarin` to be enabled.
#[component]
pub fn Xamarin(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M6.925 1.323a1.904 1.904 0 00-1.609.93L.241 11.07a1.918 1.918 0 000 1.862l5.075 8.816c.326.56.96.929 1.609.93h10.15a1.904 1.904 0 001.609-.93l5.075-8.816a1.918 1.918 0 000-1.862l-5.075-8.816a1.904 1.904 0 00-1.609-.93zm.092 5.157a.22.22 0 01.043 0h1.75a.23.23 0 01.192.114l2.97 5.292a.228.228 0 01.028.086.228.228 0 01.028-.086l2.963-5.292a.231.231 0 01.198-.114h1.751c.155.002.271.197.199.334L14.239 12l2.9 5.179c.08.138-.04.342-.199.34h-1.75a.232.232 0 01-.2-.12l-2.962-5.292A.228.228 0 0112 12.02a.228.228 0 01-.028.086l-2.97 5.292a.231.231 0 01-.192.12H7.06c-.16.002-.278-.202-.199-.34L9.761 12l-2.9-5.186c-.07-.125.015-.307.156-.334Z"
        /> < title > { title } < / title > < / svg >
    }
}
