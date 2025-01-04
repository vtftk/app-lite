
import path from 'node:path';
import { globSync } from 'glob';
import { dts } from "rollup-plugin-dts";
import { fileURLToPath } from 'node:url';
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
        input: Object.fromEntries(globSync('src/**/*.test.ts').map(file => [
            // This remove `src/` as well as the file extension from each
            // file, so e.g. src/nested/foo.js becomes nested/foo
            path.relative(
                'src',
                file.slice(0, file.length - path.extname(file).length)
            ),
            // This expands the relative paths to absolute paths, so e.g.
            // src/nested/foo becomes /project/src/nested/foo.js
            fileURLToPath(new URL(file, import.meta.url))
        ])),
        output: {
            dir: 'dist',
            format: 'es'
        },
        plugins: [typescript()],
        external: ['jest'],
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