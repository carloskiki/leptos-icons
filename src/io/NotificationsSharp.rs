#[cfg(feature = "IoNotificationsSharp")]
use leptos::*;
#[cfg(feature = "IoNotificationsSharp")]
///This icon requires the feature `IoNotificationsSharp` to be enabled.
#[component]
pub fn NotificationsSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,480a80.09,80.09,0,0,0,73.3-48H182.7A80.09,80.09,0,0,0,256,480Z" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M400,288V227.47C400,157,372.64,95.61,304,80l-8-48H216l-8,48c-68.88,15.61-96,76.76-96,147.47V288L64,352v48H448V352Z"
        /> < title > { title } < / title > < / svg >
    }
}
