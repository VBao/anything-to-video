import React, { useRef, useState } from 'react';

interface FileUploadProps {
  onFileSelect: (file: File) => void;
}

export const FileUpload: React.FC<FileUploadProps> = ({ onFileSelect }) => {
  const [fileName, setFileName] = useState<string | null>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      setFileName(file.name);
      onFileSelect(file);
    }
  };

  const handleDrop = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    const file = e.dataTransfer.files?.[0];
    if (file) {
      setFileName(file.name);
      onFileSelect(file);
    }
  };

  const preventDefaults = (e: React.DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    e.stopPropagation();
  };

  return (
    <div
      className="flex flex-col items-center justify-center w-full"
      onDragEnter={preventDefaults}
      onDragOver={preventDefaults}
      onDragLeave={preventDefaults}
      onDrop={handleDrop}
    >
      <div
        className="w-full h-48 rounded-xl bg-background shadow-neumorph flex flex-col items-center justify-center cursor-pointer hover:bg-gray-50/10 transition-colors duration-200"
        onClick={() => fileInputRef.current?.click()}
      >
        <input
          type="file"
          ref={fileInputRef}
          className="hidden"
          onChange={handleFileChange}
        />
        {fileName ? (
          <div className="text-center p-4">
            <p className="text-primary font-semibold truncate max-w-xs">{fileName}</p>
            <p className="text-xs text-gray-500 mt-1">Click to change</p>
          </div>
        ) : (
          <div className="text-center p-4 text-gray-500">
             <svg className="mx-auto h-10 w-10 mb-2" stroke="currentColor" fill="none" viewBox="0 0 48 48" aria-hidden="true">
                <path d="M28 8H12a4 4 0 00-4 4v20m32-12v8m0 0v8a4 4 0 01-4 4H12a4 4 0 01-4-4v-4m32-4l-3.172-3.172a4 4 0 00-5.656 0L28 28M8 32l9.172-9.172a4 4 0 015.656 0L28 28m0 0l4 4m4-24h8m-4-4v8m-12 4h.02" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
             </svg>
            <p className="font-medium">Click to upload or drag and drop</p>
            <p className="text-xs mt-1">Any document format</p>
          </div>
        )}
      </div>
    </div>
  );
};
