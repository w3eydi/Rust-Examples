import React from 'react';
import { useSelector } from 'react-redux';

const ProductList = () => {
    const currentCategoryName = useSelector((state) => state.currentCategory.name);
    return (
        <div>
        <h3>ProductList</h3>
        <h5>Kategori Adı: {currentCategoryName}</h5>
        </div>
    )
}

export default ProductList;