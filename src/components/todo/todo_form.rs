use yew::{Callback, InputEvent, function_component, html, Html, use_state, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct TodoFormProps {
	pub on_add: Callback<String>
}

#[function_component(TodoForm)]
pub fn todo_item(props: &TodoFormProps) -> Html {

   let title = use_state(|| "".to_string());

   let oninput = {
	   let title = title.clone();
	   Callback::from(move |e: InputEvent|{
		   let value = e.data();

		   match value {
			   Some(value) => {
				   title.set((*title).clone() + &value);
			   }
			   None => {
				   title.set("".to_string());
			   }
		   }
	   })
   };

   let onclick = {
	   let on_add = props.on_add.clone();
	   let title = title.clone();
	   Callback::from(move |e: MouseEvent| {
		   e.prevent_default();
		   title.set("".to_string());
		//    log::info!("title: {:?}", *title);
		   on_add.emit((*title).clone());
	   })
   };


    html! {
	   <form class="mb-5">
	    <div class="mb-3">
	     <label for="title" class="form-label">{"title"}</label>
	     <input type="text" value={(*title).clone()} {oninput} class="form-control" id="title" placeholder="Enter title" />
	    </div>
	    // <div class="mb-3">
	    //  {&*title}
	    // </div>
	    <button type="submit" onclick={onclick} class="btn btn-primary">{"Add"}</button>
	   </form>
    }
}
