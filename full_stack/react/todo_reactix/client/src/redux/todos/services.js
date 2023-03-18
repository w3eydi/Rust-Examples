import { createAsyncThunk } from "@reduxjs/toolkit";
import axios from "axios";

export const getTodosAsync = createAsyncThunk(
    "todos/getTodosAsync",
    async () => {
      const response = await axios(`${process.env.REACT_APP_API_BASE_ENDPOINT}/todos`);
      return response.data;
    }
  );
  
  export const addTodoAsync = createAsyncThunk('todos/addTodoAsync', async (data) => {
      const response = await axios.post(`${process.env.REACT_APP_API_BASE_ENDPOINT}/todos`, data);
      return response.data;
    }
  );
  
  export const toggleTodoAsync = createAsyncThunk('todos/toggleTodoAsync', async ({id, data}) => {
    const response = await axios.patch(`${process.env.REACT_APP_API_BASE_ENDPOINT}/todos/${id}`, data);
    return response.data;
  });