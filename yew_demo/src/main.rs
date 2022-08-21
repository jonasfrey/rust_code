use yew::prelude::*;


struct O_number_normalized{
    n_i64: i64
}

#[function_component(App)]
fn app() -> Html {
    let o_state = use_state(
        ||
        O_number_normalized {
            n_i64: 0
        }
    ); 

    let f_onclick = {
        let o_state = o_state.clone();
     
        return Callback::from(move |_| {
            o_state.set(
                O_number_normalized {
                    n_i64: o_state.n_i64 + 1
                }
            )
        });
    };

    return html! {
        <div>
            <button onclick={f_onclick}>{ "+1"}</button>
            <p>{o_state.n_i64}</p>
        </div>
    };
}

fn main() {
    yew::start_app::<App>();
}