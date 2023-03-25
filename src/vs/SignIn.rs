#[cfg(feature = "VsSignIn")]
use leptos::*;
#[cfg(feature = "VsSignIn")]
///This icon requires the feature `VsSignIn` to be enabled.
#[component]
pub fn SignIn(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M11.02 3.77l.01-.01.99.99V2.5l-.5-.5h-9l-.51.5v.493L2 3v10.29l.36.46 5 1.72L8 15v-1h3.52l.5-.5v-2.25l-1 1V13H8V4.71l-.33-.46L4.036 3h6.984v.77zM7 14.28l-4-1.34V3.72l4 1.34v9.22zm3.09-6.75h4.97v1h-4.93l1.59 1.6-.71.7-2.47-2.46v-.71l2.49-2.48.7.7-1.64 1.65z"
        /> < title > { title } < / title > < / svg >
    }
}
