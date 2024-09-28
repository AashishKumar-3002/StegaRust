import React, { useEffect, useState } from 'react';
import { TabsContent } from "@/components/ui/tabs";
import { CheckCircle, XCircle } from "lucide-react";
import { fetchStatus } from '../utils/status_api';

const StatusSection = () => {
  const [status, setStatus] = useState({
    encoding: 'loading',
    decoding: 'loading',
    metadata: 'loading'
  });

  useEffect(() => {
    const getStatus = async () => {
      const data = await fetchStatus();
      setStatus(data);
    };

    getStatus();
  }, []);

  const renderStatusIcon = (serviceStatus: string) => {
    switch (serviceStatus) {
      case 'online':
        return <CheckCircle className="text-green-500 mr-2" />;
      case 'offline':
        return <XCircle className="text-red-500 mr-2" />;
      case 'loading':
        return <span className="mr-2">Loading...</span>;
      case 'error':
        return <XCircle className="text-red-500 mr-2" />;
      default:
        return null;
    }
  };

  const getStatusText = (serviceStatus: string) => {
    console.log("Service Status:", serviceStatus); // Debug log
    return serviceStatus ? serviceStatus.charAt(0).toUpperCase() + serviceStatus.slice(1) : 'Unknown';
  };

  return (
    <TabsContent value="status">
      <h2 className="text-2xl font-bold mb-4">Service Status</h2>
      <div className="space-y-2">
        <div className="flex items-center">
          {renderStatusIcon(status.encoding)}
          <span>Encoding Service: {getStatusText(status.encoding)}</span>
        </div>
        <div className="flex items-center">
          {renderStatusIcon(status.decoding)}
          <span>Decoding Service: {getStatusText(status.decoding)}</span>
        </div>
        <div className="flex items-center">
          {renderStatusIcon(status.metadata)}
          <span>Metadata Service: {getStatusText(status.metadata)}</span>
        </div>
      </div>
    </TabsContent>
  );
};

export default StatusSection;