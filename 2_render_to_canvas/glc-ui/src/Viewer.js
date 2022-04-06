import React from 'react';
import { Grid } from '@mui/material';

export default function Viewer({canvasId}) {
    return (
        <Grid container justifyContent={"center"}>
            <Grid item>
                <canvas 
                    id={canvasId}
                    width='800px'
                    height='400px'
                />
            </Grid>
        </Grid>
    );
}