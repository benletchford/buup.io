export interface Theme {
  background: string;
  text: string;
  surface: string;
  border: string;
}

export const theme: { light: Theme; dark: Theme } = {
  light: {
    background: '#f8f9fa',
    text: 'rgba(33, 37, 41, 0.9)',
    surface: '#ffffff',
    border: '#dee2e6',
  },
  dark: {
    background: '#212529',
    text: 'rgba(248, 249, 250, 0.9)',
    surface: '#343a40',
    border: '#495057',
  },
};
