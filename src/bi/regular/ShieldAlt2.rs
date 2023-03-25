#[cfg(feature = "BiRegularShieldAlt2")]
use leptos::*;
#[cfg(feature = "BiRegularShieldAlt2")]
///This icon requires the feature `BiRegularShieldAlt2` to be enabled.
#[component]
pub fn ShieldAlt2(
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
        "M21.438 5.089a1.002 1.002 0 0 0-.959.015c-.684.389-1.355.577-2.053.577-2.035 0-3.952-1.629-5.722-3.39-.375-.373-1.063-.373-1.438 0C9.592 3.959 7.598 5.663 5.51 5.663c-.69 0-1.351-.184-2.018-.561-.298-.166-.658-.171-.96-.012s-.501.461-.528.801c-.011.129-.944 12.872 9.683 16.041a.99.99 0 0 0 .286.042H12c.097 0 .192-.014.285-.041 10.657-3.17 9.695-15.916 9.684-16.044a1 1 0 0 0-.531-.8zm-9.452 14.842c-6.979-2.255-7.934-9.412-8.014-12.477.505.14 1.019.209 1.537.209 2.492 0 4.65-1.567 6.476-3.283 1.893 1.788 3.983 3.301 6.442 3.301.53 0 1.057-.074 1.575-.22-.074 3.065-1.021 10.217-8.016 12.47z"
        /> < title > { title } < / title > < / svg >
    }
}
