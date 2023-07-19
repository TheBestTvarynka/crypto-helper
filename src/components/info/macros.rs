macro_rules! generate_algo_list_for_yew {
    (algo_list: $algo_list:expr, props: $props:expr) => {{
        let mut sorted_algo_list = $algo_list.to_vec();
        sorted_algo_list.sort();

        sorted_algo_list
            .iter()
            .map(|algo| {
                html! {
                    <option selected={ &$props.algorithm == *algo } value={*algo}>{algo}</option>
                }
            })
            .collect::<Vec<yew::virtual_dom::VNode>>()
    }};
}

pub(crate) use generate_algo_list_for_yew;
