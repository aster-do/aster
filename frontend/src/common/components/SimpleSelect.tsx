import {
  FormControl,
  InputLabel,
  MenuItem,
  Select,
  SelectChangeEvent,
  SxProps,
  Theme,
} from '@mui/material';
import { v4 as uuidv4 } from 'uuid';
import React, { useMemo } from 'react';

interface InternalOption<T> {
  id: string;
  value: T;
  label: string;
}

export interface SimpleSelectProps<T> {
  label: string;
  options: { value: T; label: string }[];
  selected?: T;
  onChange: (value: T) => void;
  sx?: SxProps<Theme>;
  variant?: 'standard' | 'outlined' | 'filled';
}

export default function SimpleSelect<T>({
  label,
  options,
  selected,
  onChange,
  sx = { minWidth: 200 },
  variant = 'standard',
}: SimpleSelectProps<T>) {
  const id = useMemo(() => uuidv4(), []);
  const internalOptions = useMemo<InternalOption<T>[]>(
    () =>
      options.map((option) => ({
        id: uuidv4(),
        value: option.value,
        label: option.label,
      })),
    [options]
  );
  const internalSelected = useMemo(
    () => internalOptions.find((option) => option.value === selected)?.id,
    [internalOptions, selected]
  );

  const handleChange = (event: SelectChangeEvent<string>) => {
    onChange(
      internalOptions.find((io) => io.id === event.target.value)?.value as T
    );
  };

  return (
    <FormControl sx={sx} size="small" variant={variant}>
      <InputLabel id={id}>{label}</InputLabel>
      <Select
        labelId={id}
        value={internalSelected}
        label="Age"
        onChange={handleChange}
      >
        {internalOptions.map((internalOption) => (
          <MenuItem key={internalOption.id} value={internalOption.id}>
            {internalOption.label}
          </MenuItem>
        ))}
      </Select>
    </FormControl>
  );
}
