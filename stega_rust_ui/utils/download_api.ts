import axios from 'axios';

// This function downloads a file from the server.
export const downloadFile = async (fileName: string) => {
    try {
        console.log(fileName)
        const response = await axios.get(`http://localhost:8000/download/${fileName}`, {
            responseType: 'blob'
        });
        const url = window.URL.createObjectURL(new Blob([response.data]));
        const link = document.createElement('a');
        link.href = url;
        link.setAttribute('download', fileName);
        document.body.appendChild(link);
        link.click();
        link.remove();
    } catch (error) {
        console.error('Error downloading file:', error);
    }
};