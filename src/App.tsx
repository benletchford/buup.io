import { ThemeProvider } from '@emotion/react';
import styled from '@emotion/styled';
import { useState } from 'react';
import { FaGithub } from 'react-icons/fa';
import buupIcon from '/buup-icon.svg';
import { TransformPanel } from './components/TransformPanel';
import { TransformSelector } from './components/TransformSelector';
import { ThemeToggle } from './components/ThemeToggle';
import { theme } from './theme';
import { loadTransformers, getTransformerOptions } from './utils/transformerLoader';

const TopBar = styled.div`
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
    min-height: 100vh;
    padding: 1.5rem;
    background: ${({ theme }) => theme.background};
    color: ${({ theme }) => theme.text};
    display: flex;
    flex-direction: column;
`;

const Header = styled.div`
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
`;

const TitleContainer = styled.div`
    display: flex;
    align-items: center;
    gap: 0.75rem;
`;

const TitleIcon = styled.img`
    height: 2rem;
    width: 2rem;
`;

const Title = styled.h1`
    margin: 0;
    font-size: 2rem;
    font-weight: 600;
`;

const PanelContainer = styled.div`
    display: grid;
    grid-template-columns: 1fr auto 1fr;
    gap: 1rem;
    flex: 1;
    min-height: 0;
`;

const SwapButton = styled.button`
    align-self: stretch;
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
        <Header>
          <TitleContainer>
            <TitleIcon src={buupIcon} alt="Buup icon" />
            <Title>Buup - the text utility belt</Title>
          </TitleContainer>
          <TopBar>
            <ThemeToggle isDark={isDark} onToggle={() => setIsDark(!isDark)} />
            <GitHubButton
              href="https://github.com/benletchford/buup"
              target="_blank"
              rel="noopener noreferrer"
            >
              <FaGithub size={20} />
            </GitHubButton>
          </TopBar>
        </Header>
        <div style={{ display: 'flex', flexDirection: 'column', flex: 1, gap: '1rem', minHeight: 0 }}>
          <TransformSelector value={transformer} onChange={setTransformer} options={options} />
          <PanelContainer>
            <TransformPanel value={input} onChange={setInput} />
            <SwapButton onClick={swapValues}>⇄</SwapButton>
            <TransformPanel value={transform(input)} readOnly />
          </PanelContainer>
        </div>
      </Container>
    </ThemeProvider>
  );
}

export default App;
