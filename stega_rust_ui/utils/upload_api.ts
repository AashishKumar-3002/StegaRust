import axios from 'axios';

interface UploadResponse {
  status: string;
  image_path: string[] | null;
  message: string;
}

export const uploadImage = async (file: File): Promise<UploadResponse> => {
  try {
    const formData = new FormData();
    formData.append('image', file);  // Key should match the expected form field name

    const response = await axios.post('https://stegarust.onrender.com/upload', formData, {
      headers: {
        'Content-Type': 'multipart/form-data'
      }
    });

    console.log(response.data);
    return response.data;
  } catch (error) {
    console.error('Error uploading image:', error);

    return {
      status: 'error',
      image_path: null,
      message: 'An error occurred while uploading the image'
    };
  }
};
