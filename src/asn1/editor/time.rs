use asn1_parser::{
    Day, GeneralizedTime, GtSecond, GtYear, Hour, LocalTimeDiffFactor, LocalTimeDirection, Minute, Month, Second,
    UtcTime, Year,
};
use web_sys::HtmlInputElement;
use yew::{Callback, Html, Properties, TargetCast, function_component, html};

use crate::common::Switch;

#[derive(PartialEq, Properties, Clone)]
pub struct GeneralizedTimeEditorProps {
    pub value: GeneralizedTime,
    pub setter: Callback<GeneralizedTime>,
}

#[function_component(GeneralizedTimeEditor)]
pub fn generalized_time_editor(props: &GeneralizedTimeEditorProps) -> Html {
    let GeneralizedTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
        local_time,
    } = props.value.clone();

    let LocalTimeDiffFactor {
        time_direction,
        hour: local_h,
        minute: local_m,
    } = local_time.unwrap_or_default();

    let setter = props.setter.clone();
    let gt = props.value.clone();
    let on_input_year = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(year) = input.value().parse::<u16>() {
            let mut new_time = gt.clone();
            new_time.year = GtYear::new(year);
            setter.emit(new_time);
        }
    });

    let setter = props.setter.clone();
    let gt = props.value.clone();
    let on_input_month = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(month) = input.value().parse::<u8>() {
            if let Ok(month) = Month::try_from(month) {
                let mut new_time = gt.clone();
                new_time.month = month;
                setter.emit(new_time);
            }
        }
    });

    let setter = props.setter.clone();
    let gt = props.value.clone();
    let on_input_day = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(day) = input.value().parse::<u8>() {
            if let Ok(day) = Day::try_from(day) {
                let mut new_time = gt.clone();
                new_time.day = day;
                setter.emit(new_time);
            }
        }
    });

    let setter = props.setter.clone();
    let gt = props.value.clone();
    let on_input_hour = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(hour) = input.value().parse::<u8>() {
            if let Ok(hour) = Hour::try_from(hour) {
                let mut new_time = gt.clone();
                new_time.hour = hour;
                setter.emit(new_time);
            }
        }
    });

    let setter = props.setter.clone();
    let gt = props.value.clone();
    let on_input_minute = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(minute) = input.value().parse::<u8>() {
            if let Ok(minute) = Minute::try_from(minute) {
                let mut new_time = gt.clone();
                new_time.minute = minute;
                setter.emit(new_time);
            }
        }
    });

    let setter = props.setter.clone();
    let gt = props.value.clone();
    let on_input_second = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(second) = input.value().parse::<f32>() {
            if let Ok(second) = GtSecond::try_from(second) {
                let mut new_time = gt.clone();
                new_time.second = second;
                setter.emit(new_time);
            }
        }
    });

    let setter = props.setter.clone();
    let gt = props.value.clone();
    let on_input_local_time_direction = Callback::from(move |time_direction: bool| {
        let mut new_time = gt.clone();

        let mut new_local_time = new_time.local_time.unwrap_or_default();
        new_local_time.time_direction = if time_direction {
            LocalTimeDirection::Plus
        } else {
            LocalTimeDirection::Minus
        };
        new_time.set_local_time(new_local_time);

        setter.emit(new_time);
    });

    let setter = props.setter.clone();
    let gt = props.value.clone();
    let on_input_local_hour = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(hour) = input.value().parse::<u8>() {
            if let Ok(hour) = Hour::try_from(hour) {
                let mut new_time = gt.clone();

                let mut new_local_time = new_time.local_time.unwrap_or_default();
                new_local_time.hour = hour;
                new_time.set_local_time(new_local_time);

                setter.emit(new_time);
            }
        }
    });

    let setter = props.setter.clone();
    let gt = props.value.clone();
    let on_input_local_minute = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(minute) = input.value().parse::<u8>() {
            if let Ok(minute) = Minute::try_from(minute) {
                let mut new_time = gt.clone();

                let mut new_local_time = new_time.local_time.unwrap_or_default();
                new_local_time.minute = minute;
                new_time.set_local_time(new_local_time);

                setter.emit(new_time);
            }
        }
    });

    html! {
        <table style="text-align: center;">
            <tr>
                <th>{"y"}</th><th>{"/"}</th><th>{"m"}</th><th>{"/"}</th><th>{"d"}</th>
                <th>{" "}</th>
                <th>{"h"}</th><th>{":"}</th><th>{"m"}</th><th>{":"}</th><th>{"s"}</th>
                <th>{" "}</th>
                <th title="local time direction">{"-/+"}</th><th title="local time hours">{"h"}</th><th>{":"}</th><th title="local time minutes">{"m"}</th>
            </tr>
            <tr>
                <td>
                    <input
                        class={"modal-input"}
                        type="number"
                        cols={"4"}
                        min={"1"}
                        max={"9999"}
                        value={year.to_string()}
                        oninput={on_input_year.clone()}
                    />
                </td>
                <td>{"/"}</td>
                <td>
                    <input
                        class={"modal-input"}
                        type="number"
                        cols={"2"}
                        min={"1"}
                        max={"12"}
                        value={month.to_string()}
                        oninput={on_input_month.clone()}
                    />
                </td>
                <td>{"/"}</td>
                <td>
                    <input
                        class={"modal-input"}
                        type="number"
                        cols={"2"}
                        min={"1"}
                        max={"31"}
                        value={day.to_string()}
                        oninput={on_input_day.clone()}
                    />
                </td>
                <td>{" "}</td>
                <td>
                    <input
                        class={"modal-input"}
                        type="number"
                        cols={"2"}
                        min={"0"}
                        max={"23"}
                        value={hour.to_string()}
                        oninput={on_input_hour.clone()}
                    />
                </td>
                <td>{":"}</td>
                <td>
                    <input
                        class={"modal-input"}
                        type="number"
                        cols={"2"}
                        min={"0"}
                        max={"59"}
                        value={minute.to_string()}
                        oninput={on_input_minute.clone()}
                    />
                </td>
                <td>{":"}</td>
                <td>
                    <input
                        class={"modal-input"}
                        type="number"
                        cols={"5"}
                        min={"0"}
                        max={"59.99"}
                        step={"0.01"}
                        value={second.to_string()}
                        oninput={on_input_second.clone()}
                    />
                </td>
                // Local time part:
                <td>{" "}</td>
                <td>
                    <Switch id={"local_time_direction_switch".to_string()} setter={on_input_local_time_direction} state={time_direction == LocalTimeDirection::Plus}/>
                </td>
                <td>
                    <input
                        class={"modal-input"}
                        type="number"
                        cols={"2"}
                        min={"0"}
                        max={"23"}
                        value={local_h.to_string()}
                        oninput={on_input_local_hour.clone()}
                    />
                </td>
                <td>{":"}</td>
                <td>
                    <input
                        class={"modal-input"}
                        type="number"
                        cols={"2"}
                        min={"0"}
                        max={"59"}
                        value={local_m.to_string()}
                        oninput={on_input_local_minute.clone()}
                    />
                </td>
            </tr>
        </table>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct UtcTimeEditorProps {
    pub value: UtcTime,
    pub setter: Callback<UtcTime>,
}

#[function_component(UtcTimeEditor)]
pub fn utc_time_editor(props: &UtcTimeEditorProps) -> Html {
    let UtcTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
    } = props.value.clone();

    let setter = props.setter.clone();
    let ut = props.value.clone();
    let on_input_year = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(year) = input.value().parse::<u8>() {
            if let Ok(year) = Year::try_from(year) {
                let mut new_time = ut.clone();
                new_time.year = year;
                setter.emit(new_time);
            }
        }
    });

    let setter = props.setter.clone();
    let ut = props.value.clone();
    let on_input_month = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(month) = input.value().parse::<u8>() {
            if let Ok(month) = Month::try_from(month) {
                let mut new_time = ut.clone();
                new_time.month = month;
                setter.emit(new_time);
            }
        }
    });

    let setter = props.setter.clone();
    let ut = props.value.clone();
    let on_input_day = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(day) = input.value().parse::<u8>() {
            if let Ok(day) = Day::try_from(day) {
                let mut new_time = ut.clone();
                new_time.day = day;
                setter.emit(new_time);
            }
        }
    });

    let setter = props.setter.clone();
    let ut = props.value.clone();
    let on_input_hour = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(hour) = input.value().parse::<u8>() {
            if let Ok(hour) = Hour::try_from(hour) {
                let mut new_time = ut.clone();
                new_time.hour = hour;
                setter.emit(new_time);
            }
        }
    });

    let setter = props.setter.clone();
    let ut = props.value.clone();
    let on_input_minute = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(minute) = input.value().parse::<u8>() {
            if let Ok(minute) = Minute::try_from(minute) {
                let mut new_time = ut.clone();
                new_time.minute = minute;
                setter.emit(new_time);
            }
        }
    });

    let setter = props.setter.clone();
    let ut = props.value.clone();
    let on_input_second = Callback::from(move |event: html::oninput::Event| {
        let input: HtmlInputElement = event.target_unchecked_into();

        if let Ok(second) = input.value().parse::<u8>() {
            if let Ok(second) = Second::try_from(second) {
                let mut new_time = ut.clone();
                new_time.second = Some(second);
                setter.emit(new_time);
            }
        }
    });

    html! {
        <table style="text-align: center;">
            <tr>
                <th>{"y"}</th><th>{"/"}</th><th>{"m"}</th><th>{"/"}</th><th>{"d"}</th>
                <th>{" "}</th>
                <th>{"h"}</th><th>{":"}</th><th>{"m"}</th><th>{":"}</th><th>{"s"}</th>
            </tr>
            <tr>
                <td>
                    <input
                        class={"modal-input"}
                        type="number"
                        cols={"2"}
                        min={"0"}
                        max={"99"}
                        value={year.to_string()}
                        oninput={on_input_year.clone()}
                    />
                </td>
                <td>{"/"}</td>
                <td>
                    <input
                        class={"modal-input"}
                        type="number"
                        cols={"2"}
                        min={"1"}
                        max={"12"}
                        value={month.to_string()}
                        oninput={on_input_month.clone()}
                    />
                </td>
                <td>{"/"}</td>
                <td>
                    <input
                        class={"modal-input"}
                        type="number"
                        cols={"2"}
                        min={"1"}
                        max={"31"}
                        value={day.to_string()}
                        oninput={on_input_day.clone()}
                    />
                </td>
                <td>{" "}</td>
                <td>
                    <input
                        class={"modal-input"}
                        type="number"
                        cols={"2"}
                        min={"0"}
                        max={"23"}
                        value={hour.to_string()}
                        oninput={on_input_hour.clone()}
                    />
                </td>
                <td>{":"}</td>
                <td>
                    <input
                        class={"modal-input"}
                        type="number"
                        cols={"2"}
                        min={"0"}
                        max={"59"}
                        value={minute.to_string()}
                        oninput={on_input_minute.clone()}
                    />
                </td>
                <td>{":"}</td>
                <td>
                    <input
                        class={"modal-input"}
                        type="number"
                        cols={"2"}
                        min={"0"}
                        max={"59"}
                        step={"1"}
                        value={second.unwrap_or_default().to_string()}
                        oninput={on_input_second.clone()}
                    />
                </td>
            </tr>
        </table>
    }
}
