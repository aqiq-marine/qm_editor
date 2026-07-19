import React from 'react';

export interface NumberFieldProps {
  label: string;
  value: number;
  min?: number;
  onChange: (value: number) => void;
}

export const NumberField: React.FC<NumberFieldProps> = ({ label, value, min, onChange }) => (
  <label>
    {label}
    <input
      type="number"
      min={min}
      value={value}
      onChange={(e) => onChange(Number(e.currentTarget.value))}
    />
  </label>
);
