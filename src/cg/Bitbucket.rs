#[cfg(feature = "CgBitbucket")]
use leptos::*;
#[cfg(feature = "CgBitbucket")]
///This icon requires the feature `CgBitbucket` to be enabled.
#[component]
pub fn Bitbucket(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M4.5831 4.63507C4.03082 4.63507 3.66768 5.07472 3.77202 5.61706L6.22804 18.383C6.33238 18.9253 6.86468 19.3649 7.41696 19.3649H16.583C17.1353 19.3649 17.6676 18.9253 17.7719 18.383L20.2279 5.61706C20.3323 5.07472 19.9691 4.63507 19.4168 4.63507H4.5831ZM13.5449 14.3649L14.4549 9.63507H9.54504L10.455 14.3649H13.5449Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
