import { configureStore } from "@reduxjs/toolkit";
import categoryListSliceName from "../components/categories/categoryListSlice";

export const store = configureStore({
    reducer: {
        currentCategory: categoryListSliceName,
    }
})