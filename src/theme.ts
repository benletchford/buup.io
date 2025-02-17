export interface Theme {
  background: string;
  text: string;
  surface: string;
  surfaceHover: string;
  border: string;
}

export const theme: { light: Theme; dark: Theme } = {
  light: {
    background: '#f8f9fa',
    text: 'rgba(33, 37, 41, 0.9)',
    surface: '#ffffff',
    surfaceHover: '#f8f9fa',
    border: '#dee2e6',
  },
  dark: {
    background: '#212529',
    text: 'rgba(248, 249, 250, 0.9)',
    surface: '#343a40',
    surfaceHover: '#2b3035',
    border: '#495057',
  },
};
