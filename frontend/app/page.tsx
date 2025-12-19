'use client';

import React, { useState } from 'react';
import { Button } from '@/components/Button';
import { FileUpload } from '@/components/FileUpload';
import { FormatSelector } from '@/components/FormatSelector';

export default function Home() {
  const [file, setFile] = useState<File | null>(null);
  const [format, setFormat] = useState('mp4');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const handleUpload = async () => {
    if (!file) {
      setError('Please select a file first.');
      return;
    }

    setLoading(true);
    setError(null);

    const formData = new FormData();
    formData.append('file', file);
    formData.append('format', format);

    try {
      // Use the proxy endpoint, or absolute URL if proxy not set yet (but we plan to set proxy)
      // Since we run on client, we need to know where the backend is.
      // We will configure a rewrite in next.config.ts to /api/convert or just /convert
      const response = await fetch('/convert', {
        method: 'POST',
        body: formData,
      });

      if (!response.ok) {
        throw new Error(`Conversion failed: ${response.statusText}`);
      }

      // Handle file download
      const blob = await response.blob();
      const url = window.URL.createObjectURL(blob);
      const a = document.createElement('a');
      a.href = url;
      // Try to get filename from header or default
      const contentDisposition = response.headers.get('Content-Disposition');
      let filename = `converted.${format}`;
      if (contentDisposition) {
        const match = contentDisposition.match(/filename="?([^"]+)"?/);
        if (match && match[1]) {
          filename = match[1];
        }
      }

      a.download = filename;
      document.body.appendChild(a);
      a.click();
      window.URL.revokeObjectURL(url);
      document.body.removeChild(a);

    } catch (err: any) {
      setError(err.message || 'An error occurred during conversion.');
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="min-h-screen flex items-center justify-center p-8 bg-background">
      <div className="w-full max-w-md bg-background rounded-3xl p-8 shadow-neumorph space-y-8">
        <div className="text-center space-y-2">
          <h1 className="text-3xl font-bold text-foreground">Converter</h1>
          <p className="text-gray-500">Anything to Video</p>
        </div>

        <div className="space-y-6">
          <FileUpload onFileSelect={setFile} />

          <FormatSelector value={format} onChange={setFormat} />

          {error && (
            <div className="p-4 rounded-xl bg-red-50 text-red-500 text-sm shadow-neumorph-pressed">
              {error}
            </div>
          )}

          <Button
            className="w-full"
            onClick={handleUpload}
            disabled={loading}
          >
            {loading ? 'Converting...' : 'Convert Now'}
          </Button>
        </div>

        <div className="text-center text-xs text-gray-400">
           Powered by Rust & Next.js
        </div>
      </div>
    </main>
  );
}
