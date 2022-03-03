use yew::prelude::*;
use components::header::Header;
use components::todo::todo_list::TodoList;
use components::todo::todo_form::TodoForm;

mod components;


#[function_component(App)]
fn app() -> Html{
    html! {
        <>
            <Header />
            <main class="container-fluid mt-2">
             <TodoForm />
             <TodoList />
            </main>
        </>

    }
}



fn main() {

    yew::start_app::<App>();

}
