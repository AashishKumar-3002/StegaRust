import axios from 'axios';

interface PrintResponse {
  status: string;
  chunks: string[] | null;
  message: string;
}

export const removeChunk = async (path: string, chunk_type: string): Promise<PrintResponse> => {
  try {
    const response = await axios.post('https://stegarust.onrender.com/remove', {
      path,
      chunk_type
    });

    console.log

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
