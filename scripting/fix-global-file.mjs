export default function fixGlobalFile() {
    return {
        name: 'fix-global-file',
        generateBundle(options, bundle) {
            // Loop through all files in the bundle
            for (const [fileName, chunk] of Object.entries(bundle)) {
                if (fileName.endsWith('.d.ts')) {
                    // Append `export {}` to the end of each .d.ts file
                    const content = chunk.code;
                    if (!content.trim().endsWith('export {}')) {
                        chunk.code = '/// <reference no-default-lib="true" />\n' + content.trim() + '\nexport {};';
                    }
                }
            }
        }
    };
}