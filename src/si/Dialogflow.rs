#[cfg(feature = "SiDialogflow")]
use leptos::*;
#[cfg(feature = "SiDialogflow")]
///This icon requires the feature `SiDialogflow` to be enabled.
#[component]
pub fn Dialogflow(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M11.996 0a1.639 1.639 0 0 0-.82.22L3.344 4.74a1.648 1.648 0 0 0-.535.498l9.136 5.28 9.213-5.32a1.652 1.652 0 0 0-.51-.458L12.818.22a1.639 1.639 0 0 0-.822-.22zm9.336 5.5l-9.387 5.422-9.3-5.373a1.648 1.648 0 0 0-.12.615v9.043a1.643 1.643 0 0 0 .819 1.42l3.918 2.266v4.617a.493.493 0 0 0 .74.424l12.654-7.303a1.639 1.639 0 0 0 .819-1.42V6.162a1.652 1.652 0 0 0-.143-.662z"
        /> < title > { title } < / title > < / svg >
    }
}
