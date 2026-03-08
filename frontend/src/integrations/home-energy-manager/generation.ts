import type {Label} from './types';

export const getLabels = async () => {
  const res = await fetch('http://localhost:8000/generation/labels', {
    method: 'GET',
  });
  if (!res.ok) {
    throw new Error('Failed to fetch labels');
  }
  return (await res.json()) as Label[];
};
