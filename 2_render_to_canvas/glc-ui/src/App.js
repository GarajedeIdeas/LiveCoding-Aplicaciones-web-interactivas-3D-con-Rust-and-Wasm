import React from "react";
import CssBaseline from '@mui/material/CssBaseline';
import { Container } from "@mui/material";
import Viewer from "./Viewer";
import { Glc, init } from 'glc-wasm';

init();

export default function App() {
    const glcRef = React.useRef();

    React.useEffect(() => {
        glcRef.current = Glc.new("glc-canvas");
    })

    return (
        <React.Fragment>
            <CssBaseline>
                <Container>
                    <Viewer canvasId='glc-canvas' />
                </Container>
            </CssBaseline>
        </React.Fragment>
    );
}
