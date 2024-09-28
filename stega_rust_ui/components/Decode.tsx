import React, { useState } from 'react';
import { TabsContent } from "@/components/ui/tabs"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { uploadImage } from '@/utils/upload_api';
import { decodeChunks } from '@/utils/decode_api';
import { Textarea } from './ui/textarea';

const DecodeSection = () => {
  const [file, setFile] = useState<File | null>(null);
  const [chunk, setChunk] = useState<string>('');
  const [decodeResponse, setDecodeResponse] = useState<string | null>(null);

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files) {
      setFile(event.target.files[0]);
    }
  };

  const handleTextareaChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setChunk(event.target.value);
  };

  const handleDecodeClick = async () => {
    if (file && chunk) {
      try {
        const response = await uploadImage(file);
        let file_path = '';
        // check if status is success then extract the image_path
        if (response.status === 'success') {
          if (typeof response.image_path === 'string') {
            file_path = response.image_path;
            const decodeResponse = await decodeChunks(file_path , chunk);
            if (decodeResponse.status === 'success') {
              const { chunks, message } = decodeResponse;
              if (chunks && chunks.length > 0) {
                setDecodeResponse(`Chunk: ${chunks[0]}\nMessage: ${message}`);
              } else {
                setDecodeResponse(`Message: ${message}`);
              }
              alert('Image successfully decoded');
            } else {
              setDecodeResponse(`Error: ${decodeResponse.message}`);
              throw new Error(decodeResponse.message);
            }
          } else {
            throw new Error('Invalid image path received');
          }
        } else {
          throw new Error(`Error uploading image: ${response.message}`);
        }
      } catch (error) {
        if (error instanceof Error) {
          setDecodeResponse(`Error: ${error.message}`);
        } else {
          setDecodeResponse('An unknown error occurred');
        }
        console.error('Error decoding image:', error);
        alert(`Error decoding image: ${error}}`);
      }
    } else {
      alert('Please provide both a file and a chunk to decode');
    }
  };

  return (
    <TabsContent value="decode">
      <h2 className="text-2xl font-bold mb-4">Retrieve Hidden Data</h2>
      <div className="space-y-4">
        <Input type="file" accept="image/*" onChange={handleFileChange} />
        <Textarea placeholder="Enter the chunk to Decode" value={chunk} onChange={handleTextareaChange} />
        <Button onClick={handleDecodeClick}>Decode</Button>
        {decodeResponse && (
          <div className="mt-4 p-4 border rounded bg-gray-100">
            <h3 className="text-lg font-semibold">Decoded Response:</h3>
            <pre className="whitespace-pre-wrap">{decodeResponse}</pre>
          </div>
        )}
      </div>
    </TabsContent>
  );
};

export default DecodeSection;