export interface SelectFieldProps<T extends string> {
  label: string;
  value: T;
  options: readonly T[];
  onChange: (value: T) => void;
}

export const SelectField = <T extends string>({ label, value, options, onChange }: SelectFieldProps<T>) => (
  <label>
    {label}
    <select value={value} onChange={(e) => onChange(e.currentTarget.value as T)}>
      {options.map((opt) => (
        <option key={opt} value={opt}>
          {opt}
        </option>
      ))}
    </select>
  </label>
);
