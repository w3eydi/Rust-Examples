import React, { useState } from "react";
import { useDispatch } from "react-redux";
import { addTodo } from "../../redux/todos/todosSlice";

function Form() {
  const [title, setTitle] = useState("");
  const dispatch = useDispatch();
  const handleSubmit = (e) => {
    if (!title) return;
    e.preventDefault();
    dispatch(addTodo({ title }));
    setTitle("");
  };
  return (
    <form onSubmit={handleSubmit}>
      <input
        className="new-todo"
        placeholder="What needs to be done?"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        autoFocus
      />
    </form>
  );
}

export default Form;
