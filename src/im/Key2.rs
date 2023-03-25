#[cfg(feature = "ImKey2")]
use leptos::*;
#[cfg(feature = "ImKey2")]
///This icon requires the feature `ImKey2` to be enabled.
#[component]
pub fn Key2(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M15.658 4.91l-1.58-1.58c-0.387-0.387-1.021-1.021-1.409-1.409l-1.58-1.58c-0.387-0.387-1.077-0.456-1.533-0.152l-4.319 2.88c-0.456 0.304-0.628 0.954-0.383 1.444l1.101 2.203c0.034 0.067 0.073 0.139 0.115 0.213l-5.571 5.571-0.5 3.5h3v-1h2v-2h2v-2h2v-1.112c0.1 0.060 0.196 0.113 0.284 0.157l2.203 1.101c0.49 0.245 1.14 0.072 1.444-0.383l2.88-4.319c0.304-0.456 0.236-1.146-0.152-1.533zM2.354 13.354l-0.707-0.707 4.868-4.868 0.707 0.707-4.868 4.868zM14.328 6.621l-0.707 0.707c-0.194 0.194-0.513 0.194-0.707 0l-4.243-4.243c-0.194-0.194-0.194-0.513 0-0.707l0.707-0.707c0.194-0.194 0.513-0.194 0.707 0l4.243 4.243c0.194 0.194 0.194 0.513 0 0.707z"
        /> < title > { title } < / title > < / svg >
    }
}
