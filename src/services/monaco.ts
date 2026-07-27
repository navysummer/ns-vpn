import type { editor } from 'monaco-editor'

let monacoConfigured = false

export const configureMonaco = async () => {
  if (monacoConfigured) return

  const monaco = await import('monaco-editor')
  const { default: EditorWorker } = await import('monaco-editor/esm/vs/editor/editor.worker?worker')
  const { default: CssWorker } = await import('monaco-editor/esm/vs/language/css/css.worker?worker')
  const { default: TsWorker } = await import('monaco-editor/esm/vs/language/typescript/ts.worker?worker')
  const { default: YamlWorker } = await import('@/utils/yaml.worker?worker')
  const { configureMonacoYaml } = await import('monaco-yaml')

  const workers: Record<string, any> = {
    css: CssWorker,
    less: CssWorker,
    scss: CssWorker,
    typescript: TsWorker,
    javascript: TsWorker,
    yaml: YamlWorker,
  }

  self.MonacoEnvironment = {
    getWorker(_, label) {
      return new (workers[label] ?? EditorWorker)()
    },
  }

  const createWebWorker = monaco.editor.createWebWorker
  type CreateWebWorker = typeof createWebWorker
  type WorkerOptions = Parameters<CreateWebWorker>[0] & { worker?: unknown }
  ;(monaco.editor as any).createWebWorker = ((options: WorkerOptions) =>
    'worker' in options
      ? createWebWorker(options)
      : monaco.createWebWorker(options)) as CreateWebWorker

  configureMonacoYaml(monaco, {
    validate: true,
    enableSchemaRequest: true,
    completion: true,
    schemas: [],
  })

  monacoConfigured = true
}
