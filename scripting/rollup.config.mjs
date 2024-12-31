
import { dts } from "rollup-plugin-dts";
import typescript from '@rollup/plugin-typescript';

import fixGlobalFile from './fix-global-file.mjs';

export default [
    {
        input: 'src/runtime.ts',
        output: {
            file: 'dist/runtime.js',
            format: 'es'
        },
        plugins: [typescript()]
    },
    {
        input: "dist/types/runtime.d.ts",
        output: [{ file: "dist/runtime.d.ts", format: "es" }],
        plugins: [
            dts({
                respectExternal: true
            }),
            fixGlobalFile(),
        ],
    },
];