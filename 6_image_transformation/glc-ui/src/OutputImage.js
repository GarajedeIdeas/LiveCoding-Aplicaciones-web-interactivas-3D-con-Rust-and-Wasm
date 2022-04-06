import { Container, Typography } from "@mui/material";
import { Box } from "@mui/system";
import React from "react";

export default function OutputImage({canvasId}) {

    return (
        <Container>
            <Typography gutterBottom>
                Output
            </Typography>
            <Box sx={{width: 300}}>
                <canvas id={canvasId} />
            </Box>
        </Container>
    );
}
