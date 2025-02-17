import styled from '@emotion/styled';
import { FC } from 'react';

const Panel = styled.div`
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  width: 100%;
`;

const TextArea = styled.textarea`
  width: 100%;
  height: 200px;
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
  return (
    <Panel>
      <TextArea
        value={value}
        onChange={e => onChange?.(e.target.value)}
        readOnly={readOnly}
        placeholder={readOnly ? 'Output will appear here...' : 'Enter text to transform...'}
      />
    </Panel>
  );
};
