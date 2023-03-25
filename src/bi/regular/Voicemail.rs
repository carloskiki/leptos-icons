#[cfg(feature = "BiRegularVoicemail")]
use leptos::*;
#[cfg(feature = "BiRegularVoicemail")]
///This icon requires the feature `BiRegularVoicemail` to be enabled.
#[component]
pub fn Voicemail(
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
        "M17.5 8a4.505 4.505 0 0 0-4.5 4.5c0 .925.281 1.784.762 2.5h-3.523c.48-.716.761-1.575.761-2.5C11 10.019 8.981 8 6.5 8S2 10.019 2 12.5 4.019 17 6.5 17c.171 0 .334-.032.5-.051V17h11v-.051c2.244-.252 4-2.139 4-4.449 0-2.481-2.019-4.5-4.5-4.5zM4 12.5C4 11.121 5.121 10 6.5 10S9 11.121 9 12.5 7.879 15 6.5 15 4 13.879 4 12.5zM17.5 15c-1.379 0-2.5-1.121-2.5-2.5s1.121-2.5 2.5-2.5 2.5 1.121 2.5 2.5-1.121 2.5-2.5 2.5z"
        /> < title > { title } < / title > < / svg >
    }
}
