import React from 'react';

interface FormatSelectorProps {
  value: string;
  onChange: (value: string) => void;
}

const formats = ['mp4', 'avi', 'gif', 'mov', 'mkv', 'webm', 'mp3'];

export const FormatSelector: React.FC<FormatSelectorProps> = ({ value, onChange }) => {
  return (
    <div className="flex flex-col gap-2">
      <label htmlFor="format" className="text-sm font-bold text-foreground ml-1">
        Target Format
      </label>
      <div className="relative">
        <select
          id="format"
          value={value}
          onChange={(e) => onChange(e.target.value)}
          className="w-full bg-background text-foreground px-4 py-3 rounded-xl shadow-neumorph-pressed focus:outline-none appearance-none cursor-pointer"
        >
          {formats.map((format) => (
            <option key={format} value={format}>
              {format.toUpperCase()}
            </option>
          ))}
        </select>
        <div className="absolute inset-y-0 right-0 flex items-center px-4 pointer-events-none text-foreground">
          <svg className="w-4 h-4 fill-current" viewBox="0 0 20 20">
            <path d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z" clipRule="evenodd" fillRule="evenodd"></path>
          </svg>
        </div>
      </div>
    </div>
  );
};
