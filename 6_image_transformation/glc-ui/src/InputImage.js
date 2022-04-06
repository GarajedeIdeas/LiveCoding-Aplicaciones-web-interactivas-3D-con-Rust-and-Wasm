import { Card, CardMedia, Container, Typography } from "@mui/material";
import { Box } from "@mui/system";
import React from "react";

export default function InputImage({imageUrl}) {

    return (
        <Container>
            <Box sx={{width: 300}}>
                <Typography>
                    Input Image
                </Typography>
                <Card sx={{maxWidth: 300}}>
                    {imageUrl && 
                        <CardMedia
                            component='img'
                            width='300'
                            image={imageUrl}
                        />}
                </Card>
            </Box>
        </Container>
    );
}
