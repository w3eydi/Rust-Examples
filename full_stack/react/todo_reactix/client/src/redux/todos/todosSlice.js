import { createSlice, createAsyncThunk } from "@reduxjs/toolkit";
import axios from "axios";
import { getTodosAsync, addTodoAsync, toggleTodoAsync } from "./services";

// re-export
export { getTodosAsync, addTodoAsync, toggleTodoAsync };

// export const getTodosAsync = createAsyncThunk('todos/getTodosAsync', async () => {
//   const response = await fetch('http://localhost:8080/todos');
//   return await response.json();
// });


// export const removeTodoAsync = createAsyncThunk('todos/remoteTodoAsync', async (id) => {
//   const response = await axios.delete(`${process.env.REACT_APP_API_BASE_ENDPOINT}/todos/${id}`);
//   return response.data;
// });
// üstteki de aynı kod
export const removeTodoAsync = createAsyncThunk('todos/remoteTodoAsync', async (id) => {
  await axios.delete(`${process.env.REACT_APP_API_BASE_ENDPOINT}/todos/${id}`);
  return id;
});

export const todosSlice = createSlice({
  name: "todos",
  initialState: {
    items: [],
    isLoading: false,
    error: null,
    activeFilter: localStorage.getItem('activeFilter'),
    addNewTodo: {
      isLoading: false,
      error: null,
    }
  },
  reducers: {
    // toggle: (state, action) => {
    //   const id = action.payload.id;
    //   const item = state.items.find((item) => item.id === id);
    //   item.completed = !item.completed;
    // },
    // destroyTodo: (state, action) => {
    //   const id = action.payload;
    //   const filtered = state.items.filter((item) => item.id !== id);
    //   state.items = filtered;
    // },
    changeActiveFilter: (state, action) => {
      state.activeFilter = action.payload;
    },
    clearCompletedTodos: (state) => {
      const filtered = state.items.filter((todo) => todo.completed === false);
      state.items = filtered;
    },
  },
  extraReducers: {
    // get todos
    [getTodosAsync.pending]: (state) => {
      state.isLoading = true;
    },
    [getTodosAsync.fulfilled]: (state, action) => {
      state.items = action.payload;
      state.isLoading = false;
    },
    [getTodosAsync.rejected]: (state, action) => {
      state.error = action.error.message;
      state.isLoading = false;
    },
    // add todo
    [addTodoAsync.pending]: (state) => {
      state.addNewTodo.isLoading = true;
    },
    [addTodoAsync.fulfilled]: (state, action) => {
      state.items.push(action.payload);
      state.addNewTodo.isLoading = false;
    },
    [addTodoAsync.rejected]: (state, action) => {
      state.addNewTodo.error = action.error.message;
      state.addNewTodo.isLoading = false;
    },
    // toggle Data
    [toggleTodoAsync.fulfilled]: (state, action) => {
      const { id, completed } = action.payload;
      const index = state.items.findIndex(item => item.id === id);
      state.items[index].completed = completed;
    },
    // remove data - alttaki ikisi de olur.
    // [removeTodoAsync.fulfilled]: (state, action) => {
    //   const { deleted } = action.payload;
    //   const filtered = state.items.filter((item) => item.id !== deleted);
    //   state.items = filtered;
    // }
    [removeTodoAsync.fulfilled]: (state, action) => {
      const deleted = action.payload;
      const filteredIndex = state.items.findIndex(item => item.id === deleted);
      state.items.splice(filteredIndex, 1);
    }
  },
});

export const selectTodos = (state) => state.todos.items;
export const selectFilteredTodos = (state) => {
  if (state.todos.activeFilter === "all") {
    return state.todos.items;
  }

  return state.todos.items.filter((todo) =>
    state.todos.activeFilter === "active"
      ? todo.completed === false
      : todo.completed === true
  );
};
export const selectActiveFilter = (state) => state.todos.activeFilter;

export const { changeActiveFilter, clearCompletedTodos } =
  todosSlice.actions;
export default todosSlice.reducer;
