#[cfg(feature = "BiSolidSkull")]
use leptos::*;
#[cfg(feature = "BiSolidSkull")]
///This icon requires the feature `BiSolidSkull` to be enabled.
#[component]
pub fn Skull(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 2C5.505 2 2 6.637 2 11c0 2.129 1.009 3.979 3 5.508V21h3v-3h2v3h4v-3h2v3h3v-4.493c1.991-1.528 3-3.379 3-5.507 0-4.363-3.505-9-10-9zM8 13c-1.121 0-2-1.098-2-2.5S6.879 8 8 8s2 1.098 2 2.5S9.121 13 8 13zm8 0c-1.121 0-2-1.098-2-2.5S14.879 8 16 8s2 1.098 2 2.5-.879 2.5-2 2.5z"
        /> < title > { title } < / title > < / svg >
    }
}
