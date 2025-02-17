import { ThemeProvider } from '@emotion/react';
import styled from '@emotion/styled';
import { useState } from 'react';
import { FaGithub } from 'react-icons/fa';
import { TransformPanel } from './components/TransformPanel';
import { TransformSelector } from './components/TransformSelector';
import { ThemeToggle } from './components/ThemeToggle';
import { theme } from './theme';
import { loadTransformers, getTransformerOptions } from './utils/transformerLoader';

const TopBar = styled.div`
    position: absolute;
    top: 0.5rem;
    right: 0.5rem;
    display: flex;
    gap: 0.5rem;
`;

const GitHubButton = styled.a`
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0.5rem;
    border-radius: 4px;
    background: ${({ theme }) => theme.surface};
    color: ${({ theme }) => theme.text};
    border: 1px solid ${({ theme }) => theme.border};
    cursor: pointer;
    text-decoration: none;
    
    &:hover {
        opacity: 0.8;
    }
`;

const Container = styled.div`
    position: relative;
    min-height: 100vh;
    padding: 0.5rem;
    background: ${({ theme }) => theme.background};
    color: ${({ theme }) => theme.text};
`;

const Title = styled.h1`
    margin: 0.5rem 0 1rem 0;
    font-size: 2rem;
    font-weight: 600;
`;

const PanelContainer = styled.div`
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    gap: 1rem;
    max-width: 1200px;
    margin: 0 auto;
`;

const SwapButton = styled.button`
    height: 200px;
    padding: 0 0.5rem;
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
        <TopBar>
          <GitHubButton
            href="https://github.com/benletchford/buup"
            target="_blank"
            rel="noopener noreferrer"
          >
            <FaGithub size={20} />
          </GitHubButton>
          <ThemeToggle isDark={isDark} onToggle={() => setIsDark(!isDark)} />
        </TopBar>
        <Title>Buup - the text utility belt</Title>
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
