import { Container, Slider, Typography } from "@mui/material";
import { Box } from "@mui/system";
import React from "react";

export default function ColorTransormation({onTransform}) {

    const handleChange = (e, v) => {
        onTransform(v);
    }

    return (
        <Container>
            <Box sx={{width: 200}}>
                <Typography gutterBottom>
                    Rotation
                </Typography>
                <Slider
                    defaultValue={0}
                    step={1}
                    min={0}
                    max={360}
                    onChange={handleChange}
                />
            </Box>
        </Container>
    );
}
