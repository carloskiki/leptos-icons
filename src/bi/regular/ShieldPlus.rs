#[cfg(feature = "BiRegularShieldPlus")]
use leptos::*;
#[cfg(feature = "BiRegularShieldPlus")]
///This icon requires the feature `BiRegularShieldPlus` to be enabled.
#[component]
pub fn ShieldPlus(
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
        "M11.63 21.91A.9.9 0 0 0 12 22a1 1 0 0 0 .41-.09C22 17.67 21 7 21 6.9a1 1 0 0 0-.55-.79l-8-4a1 1 0 0 0-.9 0l-8 4A1 1 0 0 0 3 6.9c0 .1-.92 10.77 8.63 15.01zM5 7.63l7-3.51 7 3.51c.05 2-.27 9-7 12.27C5.26 16.63 4.94 9.64 5 7.63z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.06 16h2v-3h3.01v-2h-3.01V8h-2v3h-3v2h3v3z" /> < title > { title } < / title
        > < / svg >
    }
}
