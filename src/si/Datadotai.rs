#[cfg(feature = "SiDatadotai")]
use leptos::*;
#[cfg(feature = "SiDatadotai")]
///This icon requires the feature `SiDatadotai` to be enabled.
#[component]
pub fn Datadotai(
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
        "M8.12 1.744.015 10.009 0 10.023l11.986 12.219.014.015 11.986-12.22.014-.014-8.115-8.273-.006-.006Zm1.207 1.02h5.326L11.99 5.41zm3.422 3.43 3.027-3.053L22.081 9.5h-6.054ZM8.211 3.14l3.04 3.072L7.999 9.5h-6.08Zm.62 6.977L12 6.876l3.169 3.242L12 19.842zm7.328.402h5.862l-8.793 9.005Zm-14.24 0h5.915l2.958 9.006Z"
        /> < title > { title } < / title > < / svg >
    }
}
