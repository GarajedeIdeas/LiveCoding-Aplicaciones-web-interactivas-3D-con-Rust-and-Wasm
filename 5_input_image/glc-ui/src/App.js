import React from "react";
import CssBaseline from '@mui/material/CssBaseline';
import { Container, Grid, List, ListItem } from "@mui/material";
import Viewer from "./Viewer";
import { Glc, init } from 'glc-wasm';
import InputImage from "./InputImage";
import ImageGallery from "./ImageGallery";
import { blobToImageData } from "./utils";

init();

export default function App() {
    const [inputImage, setInputImage] = React.useState(null);
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

    const handleSelectImage = ({file, url}) => {
        setInputImage(url);
        Promise.resolve(blobToImageData(file))
            .then(imageData => glcRef.current.set_input_image(imageData));
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
                    <Grid 
                        container
                        spacing={1}
                        direction='row'
                        justifyContent='center'
                        alignContent='flex-start'
                    >
                        <Grid item xs={4}>
                            <List>
                                <ListItem>
                                    <InputImage imageUrl={inputImage} />
                                </ListItem>
                            </List>
                        </Grid>
                        <Grid item xs={8}>
                            <ImageGallery onSelectImage={handleSelectImage} />
                        </Grid>
                     </Grid>
                </Container>
            </CssBaseline>
        </React.Fragment>
    );
}
