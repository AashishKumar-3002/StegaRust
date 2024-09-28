import axios from 'axios';

export const fetchStatus = async () => {
  try {
    const response = await axios.get('http://localhost:8000/status');
    return response.data;
  } catch (error) {
    console.error('Error fetching status:', error);
    return {
      encoding: 'error',
      decoding: 'error',
      metadata: 'error'
    };
  }
};