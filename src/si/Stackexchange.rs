#[cfg(feature = "SiStackexchange")]
use leptos::*;
#[cfg(feature = "SiStackexchange")]
///This icon requires the feature `SiStackexchange` to be enabled.
#[component]
pub fn Stackexchange(
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
        "M21.728 15.577v1.036c0 1.754-1.395 3.177-3.1 3.177h-.904L13.645 24v-4.21H5.371c-1.704 0-3.099-1.423-3.099-3.181v-1.032h19.456zM2.275 10.463h19.323v3.979H2.275v-3.979zm0-5.141h19.323v3.979H2.275V5.322zM18.575 0c1.681 0 3.023 1.42 3.023 3.178v1.034H2.275V3.178C2.275 1.422 3.67 0 5.375 0h13.2z"
        /> < title > { title } < / title > < / svg >
    }
}
