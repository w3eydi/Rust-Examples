import React from 'react';
import { useSelector } from 'react-redux';

function CategoryList() {
    const currentCategory = useSelector((state) => state.currentCategory.name);
    return (
        <>
            <h3>Category List</h3>
            <h5>Seçili kategori: {currentCategory}</h5>
        </>
    );
}


export default CategoryList;