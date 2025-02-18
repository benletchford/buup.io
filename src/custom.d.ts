import '@emotion/react';

declare module '@emotion/react' {
    export interface Theme {
        background: string;
        text: string;
        surface: string;
        surfaceHover: string;
        border: string;
    }
}
