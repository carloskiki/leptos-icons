#[cfg(feature = "SiV")]
use leptos::*;
#[cfg(feature = "SiV")]
///This icon requires the feature `SiV` to be enabled.
#[component]
pub fn V(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M15.583 23.4965c.0673.1924-.0435.3484-.2472.3484h-6.262c-.4075 0-.8502-.3113-.9881-.6947L.0426.7837C-.105.3925.149.1152.5276.1599l6.393.6158c.4056.0391.8441.383.9787.7675l7.6837 21.9533zM23.4736.1599l-6.393.6159c-.4055.0391-.8436.3832-.9775.7678l-3.8275 10.9895 3.6784 10.5098L23.9586.7837c.1378-.3834-.0795-.663-.485-.6238z"
        /> < title > { title } < / title > < / svg >
    }
}
