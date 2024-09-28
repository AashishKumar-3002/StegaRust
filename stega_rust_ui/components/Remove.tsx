import React, { useState } from 'react';
import { TabsContent } from "@/components/ui/tabs"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Textarea } from "@/components/ui/textarea"
import { removeChunk } from '@/utils/remove_api';
import { uploadImage } from '@/utils/upload_api';
import { downloadFile } from '@/utils/download_api';

const RemoveSection = () => {
  const [file, setFile] = useState<File | null>(null);
  const [chunk, setChunk] = useState<string>('');
  const [removeResponse, setRemoveResponse] = useState<string | null>(null);

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files) {
      setFile(event.target.files[0]);
    }
  };

  const handleTextareaChange = (event: React.ChangeEvent<HTMLTextAreaElement>) => {
    setChunk(event.target.value);
  };

  const handleRemoveClick = async () => {
    if (file && chunk) {
      try {
        let response = await uploadImage(file);
        let file_path = '';
        // check if status is success then extract the image_path
        if (response.status === 'success') {
          if (typeof response.image_path === 'string') {
            file_path = response.image_path;
            const removeResponse = await removeChunk(file_path , chunk);
            if (removeResponse.status === 'success') {
              const { chunks, message } = removeResponse;
              if (chunks && chunks.length > 0) {
                setRemoveResponse(`Chunk: ${chunks[0]}\nMessage: ${message}`);
              } else {
                setRemoveResponse(`Message: ${message}`);
              }
              downloadFile(file.name);
              alert('Hidden data removed successfully');
            } else {
              setRemoveResponse(`Error: ${removeResponse.message}`);
              throw new Error(removeResponse.message);
            }
          } else {
            throw new Error('Invalid image path received');
          }
        } else {
          throw new Error(`Error uploading image: ${response.message}`);
        }
      } catch (error) {
        console.error('Error removing hidden data:', error);
        if (error instanceof Error) {
          setRemoveResponse(`Error: ${error.message}`);
        } else {
          setRemoveResponse('An unknown error occurred');
        }
        if (error instanceof Error) {
          alert(`Error uploading image: ${error.message}`);
        } else {
          alert('An unknown error occurred');
        }
      }
    } else {
      alert('Please provide both a file and a chunk to remove');
    }
  };

  return (
    <TabsContent value="remove">
      <h2 className="text-2xl font-bold mb-4">Remove Hidden Data</h2>
      <div className="space-y-4">
        <Input type="file" accept="image/*" onChange={handleFileChange} />
        <Textarea placeholder="Enter the chunk to remove" value={chunk} onChange={handleTextareaChange} />
        <Button onClick={handleRemoveClick}>Remove Hidden Data</Button>
        {removeResponse && (
          <div className="mt-4 p-4 border rounded bg-gray-100">
            <h3 className="text-lg font-semibold">Removed Response:</h3>
            <pre className="whitespace-pre-wrap">{removeResponse}</pre>
          </div>
        )}
      </div>
    </TabsContent>
  );
};

export default RemoveSection;