import React from 'react';
import CategoryList from '../categories/CategoryList';
import ProductList from '../products/ProductList';
import Grid from '@mui/material/Grid';

function Dashboard() {
    return (
        <Grid container>
            <Grid item xs={3}>
                <CategoryList />
            </Grid>
            <Grid item xs={9}>
                <ProductList />
            </Grid>
        </Grid>
    );
}

export default Dashboard;