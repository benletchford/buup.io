export interface Transformer {
    id: string;
    title: string;
    description: string;
    transform: (input: string) => string;
    inverse?: string; // ID of the inverse transformer
}

export type TransformerModule = {
    [key: string]: Transformer;
};
