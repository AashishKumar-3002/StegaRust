import React, { useState } from 'react';
import { TabsContent } from "@/components/ui/tabs"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { uploadImage } from '@/utils/upload_api';
import { printChunks } from '@/utils/print_api';

const PrintSection = () => {
  const [file, setFile] = useState<File | null>(null);
  const [printResponse, setPrintResponse] = useState<string | null>(null);

  const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files) {
      setFile(event.target.files[0]);
    }
  };

  const handlePrintClick = async () => {
    if (file) {
      try {
        let response = await uploadImage(file);
        let file_path = '';
        // check if status is success then extract the image_path
        if (response.status === 'success') {
          if (typeof response.image_path === 'string') {
            file_path = response.image_path;
            const printResponse = await printChunks(file_path);
            if (printResponse.status === 'success') {
              const { chunks } = printResponse;
              if (chunks && chunks.length > 0) {
                setPrintResponse(`${chunks.join('\n')}\nMessage: ${printResponse.message}`);
              } else {
                setPrintResponse(`No hidden data found in image and found message : ${printResponse.message}`);
              }
              alert('Hidden chunks printed successfully');
            }
            else {
              setPrintResponse(`Error: ${printResponse.message}`);
            }
          } else {
            throw new Error('Invalid image path received');
          }
        } else {
          throw new Error(`Error printing hidden chunks: ${response.message}`);
        }
      } catch (error) {
        if (error instanceof Error) {
          setPrintResponse(`Error: ${error.message}`);
        } else {
          setPrintResponse('An unknown error occurred');
        }
        console.error('Error printing hidden chunks:', error);
        alert(`Error printing hidden chunks: ${error}}`);
      }
    } else {
      alert('Please provide a file');
    }
  };

  return (
    <TabsContent value="print">
      <h2 className="text-2xl font-bold mb-4">Print Image Metadata</h2>
      <div className="space-y-4">
        <Input type="file" accept="image/*" onChange={handleFileChange} />
        <Button onClick={handlePrintClick}>Print Metadata</Button>
        {printResponse && (
          <div className="mt-4 p-4 border rounded bg-gray-100">
            <h3 className="text-lg font-semibold">Chunks In IMG:</h3>
            <pre className="whitespace-pre-wrap">{printResponse}</pre>
          </div>
        )}
      </div>
    </TabsContent>
  );
};

export default PrintSection;