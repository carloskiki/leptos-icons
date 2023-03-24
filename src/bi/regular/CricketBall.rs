#[cfg(feature = "BiRegularCricketBall")]
use leptos::*;
#[cfg(feature = "BiRegularCricketBall")]
///This icon requires the feature `BiRegularCricketBall` to be enabled.
#[component]
pub fn CricketBall(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M19.07 4.93A10 10 0 0 0 4.93 19.07 10 10 0 0 0 19.07 4.93zM6.34 6.34a8 8 0 0 1 8.78-1.71l-.29.3.71.71.52-.53a9.53 9.53 0 0 1 .84.57L5.68 16.9a9.53 9.53 0 0 1-.57-.84l.53-.52-.71-.71-.29.29a8 8 0 0 1 1.7-8.78zm11.32 11.32a8 8 0 0 1-8.78 1.71l.29-.3-.71-.71-.52.53a9.53 9.53 0 0 1-.84-.57L18.32 7.1a9.53 9.53 0 0 1 .57.84l-.53.52.71.71.29-.29a8 8 0 0 1-1.7 8.78zm-6.37-2.12.71.7-1.41 1.42-.71-.66zm2.83-2.83.71.7-1.42 1.42-.7-.71zM17 9.88l.71.71L16.24 12l-.7-.71zm-4.29-1.42-.71-.7 1.41-1.42.71.71zm-2.83 2.83-.71-.7 1.42-1.42.7.71zm-2.83 2.83-.71-.71L7.76 12l.7.71z"
        /> < title > { title } < / title > < / svg >
    }
}
