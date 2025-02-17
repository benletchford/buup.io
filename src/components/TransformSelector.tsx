import styled from '@emotion/styled';
import { FC } from 'react';

const Container = styled.div`
    margin-bottom: 1rem;
`;

const Select = styled.select`
    width: 100%;
    padding: 0.5rem;
    border: 1px solid ${props => props.theme.border};
    border-radius: 4px;
    background: ${props => props.theme.surface};
    color: ${props => props.theme.text};
    
    &:focus {
        outline: none;
        border-color: #4dabf7;
    }
`;

interface TransformerOption {
    id: string;
    title: string;
    description: string;
}

interface TransformSelectorProps {
    value: string;
    onChange: (value: string) => void;
    options: TransformerOption[];
}

export const TransformSelector: FC<TransformSelectorProps> = ({ value, onChange, options }) => {
    return (
        <Container>
            <Select value={value} onChange={e => onChange(e.target.value)}>
                {options.map(option => (
                    <option key={option.id} value={option.id} title={option.description}>
                        {option.title}
                    </option>
                ))}
            </Select>
        </Container>
    );
};
