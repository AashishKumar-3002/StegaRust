import axios from 'axios';

interface PrintResponse {
  status: string;
  chunks: string[] | null;
  message: string;
}

export const encodeChunks = async (path: string, chunk_type: String, message: String): Promise<PrintResponse> => {
  try {
    const response = await axios.post('https://stegarust.onrender.com/encode', {
      path,
      chunk_type,
      message
    });

    return response.data;
  } catch (error) {
    console.error('Error removing chunk:', error);

    return {
      status: 'error',
      chunks: null,
      message: 'An error occurred while removing the chunk'
    };
  }
};
