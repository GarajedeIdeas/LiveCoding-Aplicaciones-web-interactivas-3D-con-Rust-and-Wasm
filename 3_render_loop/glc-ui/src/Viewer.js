import React from 'react';
import { Grid } from '@mui/material';

export default function Viewer({canvasId, onMouseMove}) {
    const [dragging, setDragging] = React.useState(false);
    const [prevCoords, setPrevCoords] = React.useState([0.0, 0.0]);

    const handleMouseDown = evt => {
        setDragging(true);
        setPrevCoords([evt.clientX, evt.clientY]);
    }

    const handleMouseUp = evt => {
        setDragging(false);
    }

    const handleMouseMove = evt => {
        if (!dragging) {
            return;
        }
        const delta = [evt.clientX - prevCoords[0], evt.clientY - prevCoords[1]];
        setPrevCoords([evt.clientX, evt.clientY]);
        onMouseMove(delta, evt.shiftKey);
    }

    return (
        <Grid container justifyContent={"center"}>
            <Grid item>
                <canvas 
                    id={canvasId}
                    width='800px'
                    height='400px'
                    onMouseDown={handleMouseDown}
                    onMouseUp={handleMouseUp}
                    onMouseMove={handleMouseMove}
                />
            </Grid>
        </Grid>
    );
}