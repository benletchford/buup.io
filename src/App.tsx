import { ThemeProvider } from '@emotion/react';
import styled from '@emotion/styled';
import { useState } from 'react';
import { TransformPanel } from './components/TransformPanel';
import { TransformSelector } from './components/TransformSelector';
import { ThemeToggle } from './components/ThemeToggle';
import { theme } from './theme';
import { loadTransformers, getTransformerOptions } from './utils/transformerLoader';

const Container = styled.div`
    min-height: 100vh;
    padding: 2rem;
    background: ${({ theme }) => theme.background};
    color: ${({ theme }) => theme.text};
`;

const Title = styled.h1`
    margin: 0 0 2rem 0;
    font-size: 2rem;
    font-weight: 600;
`;

const PanelContainer = styled.div`
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    gap: 2rem;
    max-width: 1200px;
    margin: 0 auto;
`;

const SwapButton = styled.button`
    height: 200px;
    padding: 0 1rem;
    border: 1px solid ${({ theme }) => theme.border};
    border-radius: 4px;
    background: ${({ theme }) => theme.surface};
    color: ${({ theme }) => theme.text};
    cursor: pointer;
    
    &:hover {
        background: ${({ theme }) => theme.border};
    }
`;

function App() {
  const [isDark, setIsDark] = useState(window.matchMedia('(prefers-color-scheme: dark)').matches);
  const [transformer, setTransformer] = useState('base64encode');
  const [input, setInput] = useState('Hello, world!');
  const transformers = loadTransformers();
  const options = getTransformerOptions();

  const transform = (text: string) => {
    const selectedTransformer = transformers[transformer];
    if (!selectedTransformer) {
      return 'Unknown transformer';
    }
    return selectedTransformer.transform(text);
  };

  const swapValues = () => {
    const currentTransformer = transformers[transformer];
    if (currentTransformer?.inverse) {
      setTransformer(currentTransformer.inverse);
    }
    setInput(transform(input));
  };

  return (
    <ThemeProvider theme={isDark ? theme.dark : theme.light}>
      <Container>
        <ThemeToggle isDark={isDark} onToggle={() => setIsDark(!isDark)} />
        <Title>Buup</Title>
        <TransformSelector value={transformer} onChange={setTransformer} options={options} />
        <PanelContainer>
          <TransformPanel value={input} onChange={setInput} />
          <SwapButton onClick={swapValues}>⇄</SwapButton>
          <TransformPanel value={transform(input)} readOnly />
        </PanelContainer>
      </Container>
    </ThemeProvider>
  );
}

export default App;
