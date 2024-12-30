
import typescript from '@rollup/plugin-typescript';
import { dts } from "rollup-plugin-dts";
import copy from 'rollup-plugin-copy'
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
        input: "dist/types/global.d.ts",
        output: [{ file: "dist/runtime.d.ts", format: "es" }],
        plugins: [
            copy({
                targets: [
                    { src: 'src/global.d.ts', dest: 'dist/types' },
                ]
            }),
            dts({
                respectExternal: true
            }),
            fixGlobalFile(),
        ],
    },
];