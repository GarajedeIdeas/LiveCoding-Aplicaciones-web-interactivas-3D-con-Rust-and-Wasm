import React from "react";
import CssBaseline from '@mui/material/CssBaseline';
import { Container } from "@mui/material";
import Viewer from "./Viewer";
import { Glc, init } from 'glc-wasm';

init();

export default function App() {
    const glcRef = React.useRef();
    const requestRef = React.useRef();

    const update = () => {
        glcRef.current.update();
        requestRef.current = requestAnimationFrame(update);
    }

    const handleMouseMove = (delta, modifier) => {
        if (modifier) {
            glcRef.current.move_camera(0.0, 0.0, -delta[0] * 0.01);
        } else {
            glcRef.current.move_camera(delta[0] * 0.01, delta[1] * 0.01, 0.0);
        }
    }

    React.useEffect(() => {
        glcRef.current = Glc.new("glc-canvas");
        requestRef.current = requestAnimationFrame(update);

        return () => {
            cancelAnimationFrame(requestRef.current);
        }
    }, []);

    return (
        <React.Fragment>
            <CssBaseline>
                <Container>
                    <Viewer canvasId='glc-canvas' onMouseMove={handleMouseMove}/>
                </Container>
            </CssBaseline>
        </React.Fragment>
    );
}
