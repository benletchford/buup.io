import styled from '@emotion/styled';
import { FC, useCallback } from 'react';
import { FaRegCopy } from 'react-icons/fa';

const Panel = styled.div`
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  width: 100%;
  height: 100%;
  position: relative;
`;

const TextAreaContainer = styled.div`
  position: relative;
  width: 100%;
  flex: 1;
  display: flex;
  flex-direction: column;
`;

const CopyButton = styled.button`
  position: absolute;
  top: 8px;
  right: 8px;
  padding: 6px;
  background: ${props => props.theme.surface};
  border: 1px solid ${props => props.theme.border};
  border-radius: 4px;
  color: ${props => props.theme.text};
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0.7;
  transition: opacity 0.2s, background-color 0.2s;

  &:hover {
    opacity: 1;
    background: ${props => props.theme.surfaceHover};
  }
`;

const TextArea = styled.textarea`
  width: 100%;
  flex: 1;
  padding: 0.5rem;
  border: 1px solid ${props => props.theme.border};
  border-radius: 4px;
  background: ${props => props.theme.surface};
  color: ${props => props.theme.text};
  resize: none;
  font-family: monospace;
  font-size: 1.1rem;
  
  &:focus {
    outline: none;
    border-color: #4dabf7;
  }
`;

interface TransformPanelProps {
  value: string;
  onChange?: (value: string) => void;
  readOnly?: boolean;
}

export const TransformPanel: FC<TransformPanelProps> = ({ value, onChange, readOnly }) => {
  const handleCopy = useCallback(() => {
    navigator.clipboard.writeText(value).catch(console.error);
  }, [value]);

  return (
    <Panel>
      <TextAreaContainer>
        <TextArea
          value={value}
          onChange={e => onChange?.(e.target.value)}
          readOnly={readOnly}
          placeholder={readOnly ? 'Output will appear here...' : 'Enter text to transform...'}
        />
        {value && (
          <CopyButton onClick={handleCopy} title="Copy to clipboard">
            <FaRegCopy size={16} />
          </CopyButton>
        )}
      </TextAreaContainer>
    </Panel>
  );
};
