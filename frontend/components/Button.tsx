import React from 'react';

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  children: React.ReactNode;
  variant?: 'primary' | 'secondary';
}

export const Button: React.FC<ButtonProps> = ({ children, variant = 'primary', className = '', ...props }) => {
  const baseStyles = "px-6 py-3 rounded-xl font-semibold transition-all duration-200 ease-in-out active:scale-95 focus:outline-none";

  // Neumorphism styles
  const neumorphStyles = "bg-background shadow-neumorph hover:shadow-lg active:shadow-neumorph-pressed text-foreground";
  const primaryStyles = "bg-primary text-white shadow-neumorph hover:opacity-90 active:shadow-neumorph-pressed";

  const styles = variant === 'primary' ? primaryStyles : neumorphStyles;

  return (
    <button className={`${baseStyles} ${styles} ${className}`} {...props}>
      {children}
    </button>
  );
};
