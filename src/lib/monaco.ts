import * as monaco from "monaco-editor";
// Import the workers in a production-safe way.
// This is different than in Monaco's documentation for Vite,
// but avoids a weird error ("Unexpected usage") at runtime
import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
import cssWorker from "monaco-editor/esm/vs/language/css/css.worker?worker";
import htmlWorker from "monaco-editor/esm/vs/language/html/html.worker?worker";
import jsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
import tsWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";

import libraryDefinition from "../../script/api.d.ts?raw";
import builtinLibraryDefinition from "../../script/builtin.d.ts?raw";

const compilerOptions =
  monaco.languages.typescript.javascriptDefaults.getCompilerOptions();

monaco.languages.typescript.javascriptDefaults.setCompilerOptions({
  ...compilerOptions,
  noLib: true,
});

// Built-in JS runtime library definitions (ES6 etc etc)
monaco.languages.typescript.javascriptDefaults.addExtraLib(
  builtinLibraryDefinition,
  "file:///lib.d.ts",
);

monaco.languages.typescript.javascriptDefaults.addExtraLib(
  libraryDefinition,
  "file:///global.d.ts",
);

monaco.languages.typescript.javascriptDefaults.setDiagnosticsOptions({
  noSemanticValidation: false,
  noSyntaxValidation: false,
});

self.MonacoEnvironment = {
  getWorker: function (_: string, label: string) {
    switch (label) {
      case "json":
        return new jsonWorker();
      case "css":
      case "scss":
      case "less":
        return new cssWorker();
      case "html":
      case "handlebars":
      case "razor":
        return new htmlWorker();
      case "typescript":
      case "javascript":
        return new tsWorker();
      default:
        return new editorWorker();
    }
  },
};

export default monaco;
