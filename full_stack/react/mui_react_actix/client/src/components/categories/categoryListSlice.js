import { createSlice } from "@reduxjs/toolkit";

const initialState = {
    name: "Beverages"
}

export const categoryListSlice = createSlice({
    name: 'currentCategoryName',
    initialState,
    reducers: {

    }
});

export default categoryListSlice.reducer;