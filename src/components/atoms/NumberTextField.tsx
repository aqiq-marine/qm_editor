import React from 'react';

export interface NumberTextFieldProps {
  label: string;
  value: string;
  min?: string;
  step?: string;
  onChange: (value: string) => void;
}

export const NumberTextField: React.FC<NumberTextFieldProps> = ({ label, value, min, step, onChange }) => (
  <label>
    {label}
    <input
      type="number"
      min={min}
      step={step}
      value={value}
      onChange={(e) => onChange(e.currentTarget.value)}
    />
  </label>
);
