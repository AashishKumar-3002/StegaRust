import React, { useState } from 'react';
import { TabsContent } from "@/components/ui/tabs"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { uploadImage } from '@/utils/upload_api';
import { Textarea } from './ui/textarea';
import { encodeChunks } from '@/utils/encode_api';
import { downloadFile } from '@/utils/download_api';

const EncodeSection = () => {
  const [file, setFile] = useState<File | null>(null);
  const [chunk, setChunk] = useState<string>('');
  const [message, setMessage] = useState<string>('');
  const [encodeResponse, setEncodeResponse] = useState<string | null>(null);

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files) {
      setFile(event.target.files[0]);
    }
  };

  const handleTextareaChangeChunk = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setChunk(event.target.value);
  };

  const handleTextareaChangeMesssage = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setMessage(event.target.value);
  };

  const handleEncodeClick = async () => {
    if (file && chunk && message) {
      try {
        let response = await uploadImage(file);
        let file_path = '';
        // check if status is success then extract the image_path
        if (response.status === 'success') {
          if (typeof response.image_path === 'string') {
            file_path = response.image_path;
            const encodeResponse = await encodeChunks(file_path, chunk, message);
            if (encodeResponse.status === 'success') {
              setEncodeResponse(encodeResponse.message);
              alert('Data encoded successfully');
              console.log('Encode Response:', encodeResponse);
              downloadFile(file.name);
            }
            else {
              setEncodeResponse(`Error: ${encodeResponse.message}`);
            }
          } else {
            throw new Error('Invalid image path received');
          }
        } else {
          throw new Error(`Error uploading image: ${response.message}`);
        }
      } catch (error) {
        if (error instanceof Error) {
          setEncodeResponse(`Error: ${error.message}`);
        }
        console.error('Error encoding data:', error);
        alert(`Error encoding data: ${error}`);
      }
    } else {
      alert('Please provide a file, a chunk, and a message');
    }
  };


  return (
    <TabsContent value="encode">
      <h2 className="text-2xl font-bold mb-4">Hide Data Within Image</h2>
      <div className="space-y-4">
        <Input type="file" accept="image/*" onChange={handleFileChange} />
        <Textarea placeholder="Enter your Chunk" onChange={handleTextareaChangeChunk} />
        <Textarea placeholder="Enter your secret message here" onChange={handleTextareaChangeMesssage} />
        <Button onClick={handleEncodeClick}>Encode and Download Image</Button>
        {encodeResponse && (
          <div className="mt-4 p-4 border rounded bg-gray-100">
            <h3 className="text-lg font-semibold">Encode Response:</h3>
            <pre className="whitespace-pre-wrap">{encodeResponse}</pre>
          </div>
        )}
      </div>
    </TabsContent>
  );
};

export default EncodeSection;