import React from "react";
import CssBaseline from '@mui/material/CssBaseline';
import { Container } from "@mui/material";
import { greet } from 'glc-wasm';

greet();

export default function App() {
    return (
        <React.Fragment>
            <CssBaseline>
                <Container>
                    Hello World!
                </Container>
            </CssBaseline>
        </React.Fragment>
    );
}
