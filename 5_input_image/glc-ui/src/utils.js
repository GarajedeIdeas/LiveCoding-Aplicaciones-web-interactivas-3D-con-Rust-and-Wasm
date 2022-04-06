export function blobToImageData(blob) {
    let blobUrl = URL.createObjectURL(blob);

    return new Promise((resolve, reject) => {
                let img = new Image();
                img.onload = () => resolve(img);
                img.onerror = err => reject(err);
                img.src = blobUrl;
            }).then(img => {
                URL.revokeObjectURL(blobUrl);
                let canvas = document.createElement("canvas");
                canvas.width = img.width;
                canvas.height = img.height;
                let ctx = canvas.getContext("2d");
                ctx.drawImage(img, 0, 0);
                return ctx.getImageData(0, 0, img.width, img.height);    // some browsers synchronously decode image here
            })
}
