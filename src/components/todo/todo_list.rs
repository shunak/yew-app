use yew::{function_component, html, Html, Properties};
use crate::components::todo::todo_item::TodoItem;
use crate::components::todo::types::Todo;

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
    pub todo_items: Vec<Todo>,
}


#[function_component(TodoList)]
pub fn todo_list(props: &TodoItemProps) -> Html{
	// let todo_items = vec![
	// Todo {
	// 	id: 1,
	// 	title: "Learn Rust".to_string(),
	// 	completed: false,
	// },
	// Todo {
	// 	id: 2,
	// 	title: "Learn Yew".to_string(),
	// 	completed: false,
	// },
	// Todo {
	// 	id: 3,
	// 	title: "Learn WebAssembly".to_string(),
	// 	completed: false,
	// },
	// ]; 
	html!{
		<ul class="list-group">
		{props.todo_items.iter().map(|todo| html!{
			<TodoItem title={todo.title.clone()} completed={todo.completed} />
		}).collect::<Html>()}
		</ul>
	}

}