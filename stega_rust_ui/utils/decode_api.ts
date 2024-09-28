import axios from 'axios';

interface PrintResponse {
  status: string;
  chunks: string[] | null;
  message: string;
}

export const decodeChunks = async (path: string, chunk_type: String): Promise<PrintResponse> => {
  try {
    const response = await axios.post('https://stegarust.onrender.com/decode', {
      path,
      chunk_type
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
