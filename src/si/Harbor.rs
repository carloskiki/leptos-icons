#[cfg(feature = "SiHarbor")]
use leptos::*;
#[cfg(feature = "SiHarbor")]
///This icon requires the feature `SiHarbor` to be enabled.
#[component]
pub fn Harbor(
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
        "M11.591.007C4.968.233-.218 5.785.007 12.409c.226 6.624 5.778 11.81 12.402 11.584 6.624-.232 11.809-5.786 11.584-12.411C23.762 4.963 18.211-.219 11.591.007zm.821 21.902a9.894 9.894 0 0 1-.845-.01l-.043-1.479-4.699-1.824-.096 1.776a9.886 9.886 0 0 1-4.589-7.974C1.93 6.935 6.189 2.336 11.652 2.126h.014a9.857 9.857 0 0 1 5.498 1.431l-5.17 5.673 7.52-3.686a9.906 9.906 0 0 1 1.228 1.79l-8.461 2.787 9.393-.323c.13.567.212 1.151.241 1.752l-9.314-.613 9.086 3.218c-.948 4.291-4.68 7.577-9.275 7.754zM6.686 9.038h.288v2.539H6.74v.88h.408l-.067 1.323 4.197 1.509-.127-2.832h.415v-.88h-.235V9.038h.283L9.188 6.217 6.686 9.038zm3.835 2.539h-.912V9.966h-.912v1.611h-.912V9.038h2.736v2.539zm.8 4.533l.032.725-4.352-1.563.048-.816 4.272 1.654zm-4.347-.33l4.411 1.712.032.72-4.485-1.611.042-.821zm4.48 3.088l.032.715-4.624-1.653.048-.821 4.544 1.759z"
        /> < title > { title } < / title > < / svg >
    }
}
