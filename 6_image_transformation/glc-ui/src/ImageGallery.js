import React from "react";
import { Box, Container, ImageList, ImageListItem, Typography } from "@mui/material";
import {useDropzone} from 'react-dropzone';
import FileUploadIcon from '@mui/icons-material/FileUpload';

const baseStyle = {
    flex: 1,
    display: 'flex',
    flexDirection: 'column',
    width: '200px',
    height: '150px',
    alignItems: 'center',
    padding: '20px',
    borderWidth: 2,
    borderRadius: 2,
    borderColor: '#eeeeee',
    borderStyle: 'dashed',
    backgroundColor: '#fafafa',
    color: '#bdbdbd',
    outline: 'none',
    transition: 'border .24s ease-in-out'
};

const focusedStyle = {
    borderColor: '#2196f3'
};
  
const acceptStyle = {
    borderColor: '#00e676'
};
  
const rejectStyle = {
    borderColor: '#ff1744'
};

export default function ImageGallery({onSelectImage}) {
    const [images, setImages] = React.useState({});

    const changeImage = key => {
        onSelectImage(images[key]);
    }

    const handleDrop = React.useCallback(acceptedFiles => {
        const new_images = images;
        for (const f of acceptedFiles) {
            const url = URL.createObjectURL(f);
            const image = {
                file: f,
                url
            }; 
            
            new_images[image.file.name] = image;
        }
        setImages(new_images);
        changeImage(acceptedFiles[0].name);
    }, []);

    const handleClick = event => {
        const key = event.currentTarget.dataset.id;
        changeImage(key);
    }

    const {
        getRootProps,
        getInputProps,
        isFocused,
        isDragAccept,
        isDragReject,
    } = useDropzone({
        accept: 'image/*',
        onDrop: handleDrop,
    });

    const style = React.useMemo(() => ({
        ...baseStyle,
        ...(isFocused ? focusedStyle : {}),
        ...(isDragAccept ? acceptStyle : {}),
        ...(isDragReject ? rejectStyle : {})
    }), [
        isFocused,
        isDragAccept,
        isDragReject
    ]);

    return (
        <Container>
            <Typography gutterBottom>
                Image Gallery
            </Typography>
            <ImageList cols={3} rowHeight={150} sx={{ maxHeight: 450 }}>
                {Object.entries(images).map(([key, value]) => (
                    <ImageListItem key={key}>
                        <img
                            src={value.url}
                            alt={value.file.name}
                            loading='lazy'
                            onClick={handleClick}
                            data-id={key}
                        />
                    </ImageListItem>
                ))}
            </ImageList>
            <Box {...getRootProps({style})}>
                <input {...getInputProps()} />
                <FileUploadIcon/>
                <p>Upload Image</p>
            </Box>
        </Container>
    );
}

